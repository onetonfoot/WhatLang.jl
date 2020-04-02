using WhatLang, Test, CBinding

@testset "English" begin
    text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"
    r = detect(text)
    println("_____________________")
    println(r)
    println("_____________________")
end

text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"

# r = detect(text)

# using WhatLang: CInfo, get_lib

# cinfo = CInfo(undef)
# _d = Cfunction{UInt8,Tuple{Cstring,Ref{CInfo}}}(l, :detect)

# _d(text, cinfo)

# text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"
# r = detect(text)
text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"^1000;

@time r = detect(text)
@time r = detect_lang("")