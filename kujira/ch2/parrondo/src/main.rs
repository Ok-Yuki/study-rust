use rand::Rng;

fn game_a() -> bool {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(1..=100);

    if r <= 48 {
        false
    } else {
        true
    }
}

fn game_b(current_money: &u32) -> bool {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(1..=100);

    if current_money % 3 == 0 {
        if r <= 99 {
            false
        } else {
            true
        }
    } else {
        if r <= 85 {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut current_money: u32 = 100;

    let mut rng = rand::thread_rng();
    for _ in 0..1000000 {
        let r = rng.gen_range(1..=100);
        if r <= 50 {
            match game_a() {
                true => current_money = current_money + 1,
                false => current_money = current_money - 1,
            }
        } else {
            match game_b(&current_money) {
                true => current_money = current_money + 1,
                false => current_money = current_money - 1,
            }
        }
    }
    println!("{}", current_money);
}
