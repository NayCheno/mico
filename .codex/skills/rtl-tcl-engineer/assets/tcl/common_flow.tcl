# Common Tcl utilities for batch RTL/EDA flows.
# Source this file from tool-specific scripts.

namespace eval rtlflow {
    variable default_build_dir "build"
    variable default_report_dir "reports"
}

proc rtlflow::info {message} {
    puts "INFO: $message"
}

proc rtlflow::warn {message} {
    puts stderr "WARN: $message"
}

proc rtlflow::fail {message} {
    puts stderr "ERROR: $message"
    exit 1
}

proc rtlflow::env_or_default {name default_value} {
    if {[info exists ::env($name)] && $::env($name) ne ""} {
        return $::env($name)
    }
    return $default_value
}

proc rtlflow::require_env {name} {
    if {![info exists ::env($name)] || $::env($name) eq ""} {
        rtlflow::fail "required environment variable $name is not set"
    }
    return $::env($name)
}

proc rtlflow::ensure_dir {path} {
    set norm [file normalize $path]
    file mkdir $norm
    return $norm
}

proc rtlflow::resolve_relative {base path} {
    if {[file pathtype $path] eq "absolute"} {
        return [file normalize $path]
    }
    return [file normalize [file join $base $path]]
}

proc rtlflow::read_filelist {path} {
    set norm [file normalize $path]
    if {![file exists $norm]} {
        rtlflow::fail "filelist not found: $norm"
    }

    set base [file dirname $norm]
    set files {}
    set incdirs {}
    set defines {}
    set fh [open $norm r]

    while {[gets $fh line] >= 0} {
        set line [string trim $line]
        if {$line eq ""} {
            continue
        }
        if {[string match "#*" $line] || [string match "//*" $line]} {
            continue
        }
        if {[string match "+incdir+*" $line]} {
            set dir [string range $line 8 end]
            lappend incdirs [rtlflow::resolve_relative $base $dir]
            continue
        }
        if {[string match "+define+*" $line]} {
            lappend defines [string range $line 8 end]
            continue
        }
        if {[string match "-I*" $line]} {
            set dir [string range $line 2 end]
            lappend incdirs [rtlflow::resolve_relative $base $dir]
            continue
        }

        set file [rtlflow::resolve_relative $base $line]
        if {![file exists $file]} {
            rtlflow::fail "RTL source listed but not found: $file"
        }
        lappend files $file
    }

    close $fh
    return [dict create files $files incdirs $incdirs defines $defines]
}

proc rtlflow::print_filelist_summary {parsed} {
    rtlflow::info "RTL files: [llength [dict get $parsed files]]"
    rtlflow::info "include dirs: [llength [dict get $parsed incdirs]]"
    rtlflow::info "defines: [llength [dict get $parsed defines]]"
}
