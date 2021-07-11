fn main() {
    let mut fruit = Vec::new();

    // Push values onto end of vector, type changes from generic `T` to String

    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");

    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);
    index_vec[1] += 5;
    println!("Vector: {:?}", index_vec);
}
