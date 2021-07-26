fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(1, 2));
}

#[test]
fn add_ok() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic]
fn add_fails() {
    assert_eq!(add(2, 2), 7);
}

#[test]
#[ignore = "not yet reviewed"]
fn add_negatives() {
    assert_eq!(add(-2, -2), -4)
}

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_ok() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}