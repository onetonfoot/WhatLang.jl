import Libdl
const libwhatlanglib = joinpath(@__DIR__, "libwhatlanglib.so")
function check_deps()
    global libwhatlanglib
    if !isfile(libwhatlanglib)
        error("$libwhatlanglib does not exist, Please re-run Pkg.build(\"WhatLang\"), and restart Julia.")
    end
    if Libdl.dlopen_e(libwhatlanglib) == C_NULL
        error("$libwhatlanglib cannot be opened, Please re-run Pkg.build(\"WhatLang\"), and restart Julia.")
    end
    libwhatlanglib
end
