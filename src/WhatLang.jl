module WhatLang

const deps_file = joinpath(dirname(@__FILE__), "..", "deps", "deps.jl")
if !isfile(deps_file)
    error("WhatLang.jl is not installed properly, run Pkg.build(\"WhatLang\") and restart Julia.")
end

include(deps_file)
check_deps()
include("c_api.jl")

export detect, detect_lang

end # module