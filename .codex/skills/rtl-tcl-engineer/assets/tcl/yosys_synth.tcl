# Yosys batch synthesis/check template.
# Example:
#   TOP=my_top RTL_FILELIST=filelists/rtl.f BUILD_DIR=build/yosys yosys -c scripts/yosys_synth.tcl

set script_dir [file dirname [file normalize [info script]]]
source [file join $script_dir common_flow.tcl]

set top [rtlflow::require_env TOP]
set filelist [rtlflow::require_env RTL_FILELIST]
set build_dir [rtlflow::ensure_dir [rtlflow::env_or_default BUILD_DIR "build/yosys"]]
set report_dir [rtlflow::ensure_dir [rtlflow::env_or_default REPORT_DIR [file join $build_dir reports]]]
set out_dir [rtlflow::ensure_dir [file join $build_dir out]]

set parsed [rtlflow::read_filelist $filelist]
rtlflow::print_filelist_summary $parsed

set read_args [list read_verilog -sv]
foreach incdir [dict get $parsed incdirs] {
    lappend read_args "-I$incdir"
}
foreach define [dict get $parsed defines] {
    lappend read_args "-D$define"
}

foreach rtl_file [dict get $parsed files] {
    rtlflow::info "Yosys read $rtl_file"
    yosys {*}$read_args $rtl_file
}

yosys hierarchy -check -top $top
yosys proc
yosys opt
yosys fsm
yosys opt
yosys memory
yosys opt
yosys stat

yosys write_json [file join $out_dir "${top}.json"]
yosys write_verilog -noattr [file join $out_dir "${top}_synth.v"]

rtlflow::info "Yosys outputs written under $out_dir"
rtlflow::info "Capture tool logs under $report_dir in the calling wrapper or CI system"
