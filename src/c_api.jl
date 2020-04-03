using CBinding

include("../deps/libname.jl")
include("lang.jl")
include("script.jl")

@cstruct CInfo  {
    lang::Cint
    script::Cint
    confidence::Cdouble
}

remove_nulls(text::String) = replace(text, '\0' => "")

function get_lib()
    lib_path = joinpath(dirname(@__FILE__), "..", "deps", dylib_filename())
    CBinding.Clibrary(normpath(lib_path))
end

function detect(text::String)
    c_detect = Cfunction{Cint,Tuple{Cstring,Ref{CInfo}}}(get_lib(), :detect)
    cinfo = CInfo(undef)
    return_code = c_detect(remove_nulls(text), cinfo) 

    if return_code < 0
        return (nothing, nothing, nothing)
    end

    lang = to_lang(cinfo.lang)
    script = to_script(cinfo.script)
    (lang, script, cinfo.confidence)
end


function detect_lang_whitelist(text::String, whitelist::Array{Lang})

    whitelist = map(Cint, whitelist)
    len = Cint(length(whitelist))
    @assert !isempty(whitelist) "whitelist must not be empty"
    c_detect = Cfunction{Cint,Tuple{Cstring,Ptr{Array{Cint}},Cint}}(get_lib(), :detect_lang_whitelist)
    result = text |> remove_nulls |> x->c_detect(x, whitelist, len) 

    if result < 0
        return nothing
    end

    to_lang(result)
end



function detect_lang(text::String)
    c_detect = Cfunction{Cint,Tuple{Cstring,}}(get_lib(), :detect_lang)
    result = text |> remove_nulls |> c_detect 

    if result < 0
        return nothing
    end

    to_lang(result)
end

detect_lang(text::String ; whitelist::Array{Lang} = [Lang(i) for i in 0:83]) = detect_lang_whitelist(text, whitelist)



function detect_script(text::String)
    c_detect = Cfunction{Cint,Tuple{Cstring,}}(get_lib(), :detect_script)
    result = text |> remove_nulls |> c_detect 

    if result < 0
        return nothing
    end

    to_script(result)
end