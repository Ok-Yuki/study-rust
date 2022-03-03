fn function(arr: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let len = arr.len();

    for i in 0..len {
        let mut count: i32 = 1;
        let x = &arr[i];

        for j in (1..=i).rev() {
            if arr[j - 1] < *x {
                count += 1;
            } else {
                break;
            }
        }
        for k in i + 1..len {
            println!("x: {}, k: {}", x, arr[k]);
            if arr[k] < *x {
                count += 1;
            } else {
                break;
            }
        }
        result.push(count);
    }

    return result;
}
fn main() {
    let arr: Vec<i32> = vec![3, 4, 1, 6, 2];
    let res = function(&arr);
    println!("{:?}", arr);
}
