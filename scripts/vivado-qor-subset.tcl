set script_dir [file dirname [file normalize [info script]]]
set repo_root [file normalize [file join $script_dir ".."]]

if {[info exists ::env(MICO_VIVADO_REPORT_DIR)]} {
  set report_root [file normalize $::env(MICO_VIVADO_REPORT_DIR)]
} else {
  set report_root [file normalize [file join $repo_root "build" "reports" "vivado-host"]]
}
file mkdir $report_root
set report_root_json [string map {"\\" "/"} $report_root]
set repo_root_json [string map {"\\" "/"} $repo_root]
if {[string first "${repo_root_json}/" $report_root_json] == 0} {
  set report_root_json [string range $report_root_json [expr {[string length $repo_root_json] + 1}] end]
}

# Representative host-Vivado subset for the DAC 2027 audit flow. The script
# writes measurement-only copies under build/reports; it never modifies source
# RTL or benchmark wrappers.
set part_name "xc7a35tcpg236-1"
set clock_period_ns "10.000"
set vivado_flow "out_of_context_synth_measurement_copy"
set top_copy_policy "build-only sanitized top copies add mico_observe plus KEEP/DONT_TOUCH attributes; source RTL is unchanged"

set tasks {
  {T001_stream_fifo rtl/examples/mico_example_leafs.sv build/bench/T001_stream_fifo/top.sv benchmarks/qor/reference/T001_stream_fifo_ref.sv {clk}}
  {T003_width_adapter rtl/examples/mico_example_leafs.sv build/bench/T003_width_adapter/top.sv benchmarks/qor/reference/T003_width_adapter_ref.sv {clk}}
  {T058_streaming_accelerator_case rtl/case_studies/mico_case_studies.sv build/bench/T058_streaming_accelerator_case/top.sv benchmarks/qor/reference/T058_streaming_accelerator_case_ref.sv {clk}}
  {T063_axi_apb_wrapper_case rtl/case_studies/mico_case_studies.sv build/bench/T063_axi_apb_wrapper_case/top.sv benchmarks/qor/reference/T063_axi_apb_wrapper_case_ref.sv {clk}}
}

proc json_escape {value} {
  set out [string map {
    "\\" "\\\\"
    "\"" "\\\""
    "\n" "\\n"
    "\r" "\\r"
    "\t" "\\t"
  } $value]
  return $out
}

proc json_value {value} {
  if {$value eq "" || $value eq "null"} {
    return "null"
  }
  if {[string is double -strict $value] || [string is integer -strict $value]} {
    return $value
  }
  if {$value eq "true" || $value eq "false"} {
    return $value
  }
  return "\"[json_escape $value]\""
}

proc count_cells {patterns} {
  set count 0
  foreach cell [get_cells -hier -quiet] {
    set ref_name [get_property REF_NAME $cell]
    foreach pattern $patterns {
      if {[string match $pattern $ref_name]} {
        incr count
        break
      }
    }
  }
  return $count
}

proc add_vivado_observe_port {text} {
  set lines [split $text "\n"]
  set out {}
  set in_top 0
  set in_port_list 0
  set wires {}

  foreach line $lines {
    set current $line

    if {[regexp {^\s*module\s+Top\s*\(} $line]} {
      set in_top 1
      set in_port_list 1
    } elseif {$in_top && $in_port_list && [regexp {^\s*input\s+logic\s+rst\s*$} $line]} {
      regsub {rst\s*$} $line {rst,} current
      lappend out $current
      lappend out "  output logic mico_observe"
      continue
    } elseif {$in_top && $in_port_list && [regexp {^\s*\);\s*$} $line]} {
      set in_port_list 0
    }

    if {$in_top && !$in_port_list && [regexp {^(\s*)logic\s+(\[[^]]+\]\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*;} $line -> indent range name]} {
      lappend wires $name
      set current "${indent}(* KEEP = \"true\" *) [string trimleft $line]"
    } elseif {$in_top && !$in_port_list && [regexp {^(\s*)([A-Za-z_][A-Za-z0-9_]*)\s+([A-Za-z_][A-Za-z0-9_]*)\s+\($} $line -> indent module_name inst_name]} {
      set current "${indent}(* DONT_TOUCH = \"true\", KEEP_HIERARCHY = \"yes\" *) ${module_name} ${inst_name} ("
    } elseif {$in_top && [regexp {^\s*endmodule\s*$} $line]} {
      if {[llength $wires] == 0} {
        set observe_expr "rst"
      } else {
        set observe_expr "^{[join $wires ", "]}"
      }
      lappend out ""
      lappend out "  // Measurement-only observability point for Vivado QoR subset."
      lappend out "  assign mico_observe = ${observe_expr};"
      lappend out $current
      set in_top 0
      continue
    }

    lappend out $current
  }

  return [join $out "\n"]
}

proc sanitize_sv_for_vivado {src dst {observe_top 0}} {
  set in [open $src r]
  set text [read $in]
  close $in
  set text [string map {"`default_nettype none" "`default_nettype wire"} $text]
  if {$observe_top} {
    set text [add_vivado_observe_port $text]
  }
  set out [open $dst w]
  puts -nonewline $out $text
  close $out
}

proc timing_record {} {
  set paths [get_timing_paths -max_paths 1 -quiet]
  if {[llength $paths] == 0} {
    return [dict create status no_timing_paths wns null pass null]
  }
  set path [lindex $paths 0]
  set slack [get_property SLACK $path]
  set pass [expr {$slack >= 0.0 ? "true" : "false"}]
  return [dict create status timed wns $slack pass $pass]
}

proc run_case {repo_root report_root part_name clock_period_ns task kind rtl wrapper clocks} {
  set case_name "${task}_${kind}"
  set case_dir [file join $report_root $case_name]
  file mkdir $case_dir
  set sanitized_rtl [file join $case_dir "rtl.sv"]
  set sanitized_wrapper [file join $case_dir "top.sv"]
  sanitize_sv_for_vivado [file join $repo_root $rtl] $sanitized_rtl
  sanitize_sv_for_vivado [file join $repo_root $wrapper] $sanitized_wrapper 1
  create_project -in_memory -part $part_name
  set status pass
  set error_text ""

  if {[catch {
    read_verilog -sv $sanitized_rtl
    read_verilog -sv $sanitized_wrapper
    synth_design -top Top -part $part_name -mode out_of_context -flatten_hierarchy none
    foreach clock_name $clocks {
      set ports [get_ports -quiet $clock_name]
      if {[llength $ports] > 0} {
        create_clock -period $clock_period_ns -name $clock_name $ports
        set data_inputs [get_ports -quiet rst]
        if {[llength $data_inputs] > 0} {
          set_input_delay -clock $clock_name 0.000 $data_inputs
        }
        set observe_outputs [get_ports -quiet mico_observe]
        if {[llength $observe_outputs] > 0} {
          set_output_delay -clock $clock_name 0.000 $observe_outputs
        }
      }
    }
    report_utilization -file [file join $case_dir "utilization.rpt"]
    report_timing_summary -file [file join $case_dir "timing_summary.rpt"]
  } err]} {
    set status failed
    set error_text $err
  }

  set lut_count 0
  set ff_count 0
  set bram_count 0
  set dsp_count 0
  set timing [dict create status not_run wns null pass null]
  if {$status eq "pass"} {
    set lut_count [count_cells {LUT*}]
    set ff_count [count_cells {FD*}]
    set bram_count [count_cells {RAMB*}]
    set dsp_count [count_cells {DSP*}]
    set timing [timing_record]
  }
  close_project

  return [dict create \
    task $task \
    kind $kind \
    status $status \
    lut $lut_count \
    ff $ff_count \
    bram $bram_count \
    dsp $dsp_count \
    wns [dict get $timing wns] \
    timing_status [dict get $timing status] \
    timing_pass [dict get $timing pass] \
    error $error_text]
}

proc find_record {records task kind} {
  foreach record $records {
    if {[dict get $record task] eq $task && [dict get $record kind] eq $kind} {
      return $record
    }
  }
  return ""
}

proc pct_delta {generated reference} {
  if {$reference == 0} {
    if {$generated == 0} {
      return 0.0
    }
    return "null"
  }
  return [expr {(double($generated) - double($reference)) * 100.0 / double($reference)}]
}

set records {}
foreach task_info $tasks {
  lassign $task_info task rtl generated_wrapper reference_wrapper clocks
  lappend records [run_case $repo_root $report_root $part_name $clock_period_ns $task generated $rtl $generated_wrapper $clocks]
  lappend records [run_case $repo_root $report_root $part_name $clock_period_ns $task reference $rtl $reference_wrapper $clocks]
}

set deltas {}
foreach task_info $tasks {
  lassign $task_info task rtl generated_wrapper reference_wrapper clocks
  set gen [find_record $records $task generated]
  set ref [find_record $records $task reference]
  if {$gen eq "" || $ref eq "" || [dict get $gen status] ne "pass" || [dict get $ref status] ne "pass"} {
    lappend deltas [dict create task $task status not_available lut_delta_pct null ff_delta_pct null bram_delta_pct null dsp_delta_pct null]
  } else {
    lappend deltas [dict create \
      task $task \
      status available \
      lut_delta_pct [pct_delta [dict get $gen lut] [dict get $ref lut]] \
      ff_delta_pct [pct_delta [dict get $gen ff] [dict get $ref ff]] \
      bram_delta_pct [pct_delta [dict get $gen bram] [dict get $ref bram]] \
      dsp_delta_pct [pct_delta [dict get $gen dsp] [dict get $ref dsp]]]
  }
}

set csv_path [file join $report_root "vivado_qor_subset_summary.csv"]
set csv [open $csv_path w]
puts $csv "task,kind,status,lut,ff,bram,dsp,wns,timing_status,timing_pass"
foreach record $records {
  puts $csv "[dict get $record task],[dict get $record kind],[dict get $record status],[dict get $record lut],[dict get $record ff],[dict get $record bram],[dict get $record dsp],[dict get $record wns],[dict get $record timing_status],[dict get $record timing_pass]"
}
close $csv

set delta_csv_path [file join $report_root "vivado_qor_subset_delta.csv"]
set delta_csv [open $delta_csv_path w]
puts $delta_csv "task,status,lut_delta_pct,ff_delta_pct,bram_delta_pct,dsp_delta_pct"
foreach delta $deltas {
  puts $delta_csv "[dict get $delta task],[dict get $delta status],[dict get $delta lut_delta_pct],[dict get $delta ff_delta_pct],[dict get $delta bram_delta_pct],[dict get $delta dsp_delta_pct]"
}
close $delta_csv

set json_path [file join $report_root "vivado_qor_subset_summary.json"]
set json [open $json_path w]
puts $json "{"
puts $json "  \"schema_version\": \"mico.vivado_qor_subset.v0\","
puts $json "  \"vivado_part\": \"[json_escape $part_name]\","
puts $json "  \"vivado_flow\": \"[json_escape $vivado_flow]\","
puts $json "  \"clock_period_ns\": [json_value $clock_period_ns],"
puts $json "  \"top_copy_policy\": \"[json_escape $top_copy_policy]\","
puts $json "  \"report_root\": \"[json_escape $report_root_json]\","
puts $json "  \"records\": \["
for {set i 0} {$i < [llength $records]} {incr i} {
  set record [lindex $records $i]
  puts $json "    {"
  set keys {task kind status lut ff bram dsp wns timing_status timing_pass error}
  for {set j 0} {$j < [llength $keys]} {incr j} {
    set key [lindex $keys $j]
    set comma [expr {$j + 1 < [llength $keys] ? "," : ""}]
    puts $json "      \"${key}\": [json_value [dict get $record $key]]${comma}"
  }
  set comma [expr {$i + 1 < [llength $records] ? "," : ""}]
  puts $json "    }${comma}"
}
puts $json "  ],"
puts $json "  \"deltas\": \["
for {set i 0} {$i < [llength $deltas]} {incr i} {
  set delta [lindex $deltas $i]
  puts $json "    {"
  set keys {task status lut_delta_pct ff_delta_pct bram_delta_pct dsp_delta_pct}
  for {set j 0} {$j < [llength $keys]} {incr j} {
    set key [lindex $keys $j]
    set comma [expr {$j + 1 < [llength $keys] ? "," : ""}]
    puts $json "      \"${key}\": [json_value [dict get $delta $key]]${comma}"
  }
  set comma [expr {$i + 1 < [llength $deltas] ? "," : ""}]
  puts $json "    }${comma}"
}
puts $json "  ]"
puts $json "}"
close $json

set failed 0
foreach record $records {
  if {[dict get $record status] ne "pass"} {
    set failed 1
  }
}

puts "wrote $json_path"
puts "wrote $csv_path"
puts "wrote $delta_csv_path"
if {$failed} {
  error "one or more Vivado QoR subset cases failed"
}
