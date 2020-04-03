using WhatLang, Test 

@testset "detect_lang" begin
    text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"
    lang = detect_lang(text)
    @test lang == Lang(1)
end

@testset "detect_script" begin
    text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"
    script = detect_script(text)
    @test script == Script(0)
end

@testset "detect" begin
    text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"
    (lang, script, confidence) = detect(text)
    @test lang == Lang(1)
    @test script == Script(0)
end


# TODO Benchmark
# text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"^100000;
# @time r = detect(text)
# @time r = detect_lang(text)