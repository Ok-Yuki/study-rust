#[derive(Debug)]
struct Counter {
    length: u32,
    count: u32,
}

impl Counter {
    fn new(length: u32) -> Counter {
        Counter {
            count: 0,
            length,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {

        if self.count < self.length {
            self.count += 1;
            Some(self.count)
        } else  {
            None
        }
    }
}

fn main() {
    for i in Counter::new(12) {
        println!("{}", i)
    }
}