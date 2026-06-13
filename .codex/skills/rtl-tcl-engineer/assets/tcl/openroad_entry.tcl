# OpenROAD design entry template.
# Example:
#   DESIGN_NAME=my_top NETLIST=build/yosys/out/my_top_synth.v SDC=constraints/clocks.sdc openroad -exit scripts/openroad_entry.tcl

set script_dir [file dirname [file normalize [info script]]]
source [file join $script_dir common_flow.tcl]

set design_name [rtlflow::require_env DESIGN_NAME]
set netlist [file normalize [rtlflow::require_env NETLIST]]
set build_dir [rtlflow::ensure_dir [rtlflow::env_or_default BUILD_DIR "build/openroad"]]
set report_dir [rtlflow::ensure_dir [rtlflow::env_or_default REPORT_DIR [file join $build_dir reports]]]

if {![file exists $netlist]} {
    rtlflow::fail "netlist not found: $netlist"
}

if {[info exists ::env(TECH_LEF)] && $::env(TECH_LEF) ne ""} {
    read_lef -tech [file normalize $::env(TECH_LEF)]
}

if {[info exists ::env(SC_LEF)] && $::env(SC_LEF) ne ""} {
    foreach lef $::env(SC_LEF) {
        read_lef [file normalize $lef]
    }
}

if {[info exists ::env(LIBERTY)] && $::env(LIBERTY) ne ""} {
    foreach lib $::env(LIBERTY) {
        read_liberty [file normalize $lib]
    }
}

read_verilog $netlist
link_design $design_name

if {[info exists ::env(SDC)] && $::env(SDC) ne ""} {
    set sdc [file normalize $::env(SDC)]
    if {![file exists $sdc]} {
        rtlflow::fail "SDC file not found: $sdc"
    }
    read_sdc $sdc
} else {
    rtlflow::warn "SDC is not set; timing reports may be meaningless"
}

write_db [file join $build_dir "${design_name}_entry.odb"]

catch {report_design_area > [file join $report_dir "${design_name}_area.rpt"]}
catch {report_checks -path_delay max > [file join $report_dir "${design_name}_setup.rpt"]}
catch {report_checks -path_delay min > [file join $report_dir "${design_name}_hold.rpt"]}

rtlflow::info "OpenROAD entry database written under $build_dir"
