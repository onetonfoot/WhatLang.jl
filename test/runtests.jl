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

@testset "whitelist" begin
    text = "Hello my name is Domininc." 
    @test_nowarn detect_lang(text, whitelist = [Lang(1), Lang(2)])
end

@testset "edge cases" begin

    # handles embedded nulls
    @test_nowarn detect_lang("akds\0jlas\0kadsfkjl")

end
# text = "Hello how are you doing? It would be great if this rust library would work! Oh my god"^100000;
# @benchmark detect(text)
# @time r = detect_lang(text)