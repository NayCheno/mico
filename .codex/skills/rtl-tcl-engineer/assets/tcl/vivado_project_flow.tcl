# Vivado project-mode synthesis template.
# Example:
#   TOP=my_top PART=xc7a35tcsg324-1 RTL_FILELIST=filelists/rtl.f XDC=constraints/board.xdc vivado -mode batch -source scripts/vivado_project_flow.tcl

set script_dir [file dirname [file normalize [info script]]]
source [file join $script_dir common_flow.tcl]

set top [rtlflow::require_env TOP]
set part [rtlflow::require_env PART]
set filelist [rtlflow::require_env RTL_FILELIST]
set build_dir [rtlflow::ensure_dir [rtlflow::env_or_default BUILD_DIR "build/vivado"]]
set report_dir [rtlflow::ensure_dir [rtlflow::env_or_default REPORT_DIR [file join $build_dir reports]]]
set jobs [rtlflow::env_or_default JOBS 4]
set project_name [rtlflow::env_or_default PROJECT_NAME $top]

set parsed [rtlflow::read_filelist $filelist]
rtlflow::print_filelist_summary $parsed

create_project -force $project_name $build_dir -part $part

foreach rtl_file [dict get $parsed files] {
    rtlflow::info "Vivado add $rtl_file"
    add_files -norecurse $rtl_file
}

set incdirs [dict get $parsed incdirs]
if {[llength $incdirs] > 0} {
    set_property include_dirs $incdirs [current_fileset]
}

foreach define [dict get $parsed defines] {
    lappend vivado_defines $define
}
if {[info exists vivado_defines] && [llength $vivado_defines] > 0} {
    set_property verilog_define $vivado_defines [current_fileset]
}

set_property top $top [current_fileset]

if {[info exists ::env(XDC)] && $::env(XDC) ne ""} {
    set xdc [file normalize $::env(XDC)]
    if {![file exists $xdc]} {
        rtlflow::fail "XDC file not found: $xdc"
    }
    add_files -fileset constrs_1 -norecurse $xdc
}

update_compile_order -fileset sources_1
launch_runs synth_1 -jobs $jobs
wait_on_run synth_1
open_run synth_1

report_utilization -file [file join $report_dir "${top}_utilization.rpt"]
report_timing_summary -file [file join $report_dir "${top}_timing_summary.rpt"]
write_checkpoint -force [file join $build_dir "${top}_synth.dcp"]

rtlflow::info "Vivado reports written under $report_dir"
