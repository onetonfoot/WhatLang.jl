using CBinding

include("../deps/libname.jl")
include("lang.jl")
include("script.jl")

@cstruct CInfo  {
    lang::Cint
    script::Cint
    confidence::Cdouble
}

function get_lib()
    lib_path = joinpath(dirname(@__FILE__), "..", "deps", dylib_filename())
    CBinding.Clibrary(normpath(lib_path))
end

function detect(text::String)
    c_detect = Cfunction{Cint,Tuple{Cstring,Ref{CInfo}}}(get_lib(), :detect)
    cinfo = CInfo(undef)
    return_code = c_detect(text, cinfo) 

    if return_code < 0
        return (nothing, nothing, nothing)
    end

    lang = to_lang(cinfo.lang)
    script = to_script(cinfo.script)
    (lang, script, cinfo.confidence)
end


function detect_lang(text::String)
    c_detect = Cfunction{Cint,Tuple{Cstring,}}(get_lib(), :detect_lang)
    result = c_detect(text) 

    if result < 0
        return nothing
    end

    to_lang(result)
end


function detect_script(text::String)
    c_detect = Cfunction{Cint,Tuple{Cstring,}}(get_lib(), :detect_script)
    result = c_detect(text) 

    if result < 0
        return nothing
    end

    to_script(result)
end