fn use_iter() {
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
}

fn use_range() {
    for number in 0..5 {
        println!("{}", number * 2);
    }
}

fn main() {
    use_iter();
    use_range();
}
