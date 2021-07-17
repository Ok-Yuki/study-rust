fn main() {
    let mut greeting = String::from("hello");
    change(&mut greeting);
}

fn change(text: &mut String) {
    text.push_str(", world");
}