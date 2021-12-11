
pub fn run() {
    let r;
    {
        let x = String::from("hello");
        r = &x;
        println!("{}", r);
    }
    // 所有権をもつxが上のブロック終了時に開放され、
    // ダングリングポインタが発生するため、コンパイルエラー
    // println!("{}", r);
}
