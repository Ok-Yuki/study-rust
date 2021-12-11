pub mod sub_a;
pub mod sub_b;


pub fn run(){
    let i1 = 1;
    let i2 = i1;

    println!("{:p}", &i1);
    println!("{:p}", &i2);

}
