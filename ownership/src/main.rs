fn process(s: String) {
    println!("{}", s);
}


fn main() {
    let s = String::from("Hello, world!");
    // process(s); とした場合、10行目でコンパイルエラー
    process(s.clone());
    process(s);
}
