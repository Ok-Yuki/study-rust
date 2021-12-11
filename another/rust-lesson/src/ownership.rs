pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    

    let i1 = 1;
    let i2 = i1;
    println!("{} {:p}", i1, &i1);
    println!("{} {:p}", i2, &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {:p}", sl1, &sl1);
    println!("{} {:p}", sl2, &sl2);


    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{:p}", &s3);
    println!("{:p}", &s4);
    println!("{:?}", s3.as_ptr());
    println!("{:?}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    println!("stack_addres: {:p}", &s5);
    println!("heap_addres: {:?}", s5.as_ptr());
    println!("len: {}", s5.len());
    println!("cap: {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5);
    
    let s6 = String::from("hello");
    println!("stack_addres: {:p}", &s6);
    println!("heap_addres: {:?}", s6.as_ptr());
    println!("len: {}", s6.len());
    let s7 = take_giveback_ownership(s6);
    println!("stack_addres: {:p}", &s7);
    println!("heap_addres: {:?}", s7.as_ptr());
    println!("len: {}", s7.len());

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("{}: {}", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);


    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {}", r1, r2);
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r1 = &s12;
    println!("{} and {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);

    let s1 = String::from("hellow");
    let sr1 = &s1;
    let sr2 = &s1;
    println!("stack_address {:p}", &s1);
    println!("heap_memory_address {:?}", s1.as_ptr());
    println!("len {}", s1.len());
    println!("capacity {}", s1.capacity());
    println!("reference {:p}", sr1);
    println!("reference {:p}", sr2);
    println!("stack_address {:p}", &sr1);
    println!("stack_address {:p}", &sr2);


}

fn take_ownership(s: String) {
    println!("stack_addres: {:p}", &s);
    println!("heap_addres: {:?}", s.as_ptr());
    println!("len: {}", s.len());
    println!("cap: {}", s.capacity());
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
