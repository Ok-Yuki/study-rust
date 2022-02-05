fn kakuritu(rate: f32, count: u32) -> f32 {
    1.0 - ((1.0 - rate).powf(count as f32))
}

fn main() {
    let result = kakuritu(0.5, 2);
    println!("{}", result);
}
