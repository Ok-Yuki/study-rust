#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: &String) {}

fn main() {
    let text = String::from("The quick brawn fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    // erase(text);　だとコンパイルエラーfox,dogが参照している
    // textオブジェクトが関数に委譲されるため
    erase(&text);

    println!("{:?}", fox);
    println!("{:?}", dog);
}