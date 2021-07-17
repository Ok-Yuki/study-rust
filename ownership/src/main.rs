fn process(s: String) {
    println!("{}", s);
}


fn main() {
    let s = String::from("Hello, world!");
    // process(s); とした場合、11行目でコンパイルエラー
    // cloneを使用して複製すれば回避できるが、オーバーヘッドがかかる。
    process(s.clone());
    process(s);
}
