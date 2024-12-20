fn greet_world(){
    let england = "Hello, World!";
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let chinese = "你好，世界";
    let arabic = "مرحبا بالعالم";
    let russian = "Здравствуй, мир";
    let korean = "안녕하세요, 세계";
    let hindi = "नमस्ते दुनिया";
    let french = "Bonjour le monde";
    let spanish = "Hola, mundo";
    let italian = "Ciao, mondo";
    let portuguese = "Olá, mundo";
    let swedish = "Hej, världen";
    let norwegian = "Hallo, verden";
    let danish = "Hej, verden";
    let dutch = "Hallo, wereld";
    let polish = "Witaj, świecie";
    let hungarian = "Helló, világ";
    let turkish = "Merhaba, dünya";
    let tailand = "สวัสดี, โลก";
    let vietnamese = "Chào thế giới";
    let greek = "Γειά σου, κόσμε";
    let hebrew = "שלום עולם";
    let persian = "سلام دنیا";
    let urdu = "ہیلو، دنیا";
    let regions = [england, southern_germany, japan, chinese, arabic, russian, korean, hindi, french, spanish, italian, portuguese, swedish, norwegian, danish, dutch, polish, hungarian, turkish, tailand, vietnamese, greek, hebrew, persian, urdu];
    for region in regions.iter(){
        println!("{}", &region);
    }
}
fn main() {
    greet_world();
}
