const rustprojname = "whatlang-bindings"
const rustlibname = "whatlanglib"
const juliapackage = "WhatLang"

# Windows .dlls do not have the "lib" prefix
const libname = Sys.iswindows() ? rustlibname : "lib" * rustlibname

function dylib_filename()
    @static if Sys.isapple()
        "$libname.dylib"
    elseif Sys.islinux()
        "$libname.so"
    elseif Sys.iswindows()
        "$libname.dll"
    else
        error("Not supported: $(Sys.KERNEL)")
    end
end