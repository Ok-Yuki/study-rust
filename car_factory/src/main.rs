#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (String, u32)
}

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

fn car_quality (miles: u32) -> (String, u32) {

    let mut quality: (String, u32) = ("New".to_string(), 0);

    if miles > 0 {
        quality = ("Used".to_string(), miles);
    }

    return quality
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments

    let car = Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    };

    // Return new instance of "Car" struct
    return car
}

fn main() {
    use std::collections::HashMap;

    let mut orders: HashMap<String, u32> = HashMap::new();
    let (mut new_cars, mut used_cars) = (1, 1);
    let (mut manual, mut auto) = (1, 1);

    let colors = ["Blue", "Green", "Red", "Silver"];

    let (mut index, mut order) = (1, 1);

    let mut car: Car;
    let mut miles = 1000;
    let mut roof = true;
    let mut engine: Transmission;

    while order < 12 {

        if order % 3 == 0 {
            engine = Transmission::Automatic;

            orders.insert("Automatic".to_string(), auto);
            auto = auto + 1;

            roof = !roof;
        } else if order % 2 == 0 {
            engine = Transmission::SemiAuto;
        } else {
            engine = Transmission::Manual;

            orders.insert("Manual".to_string(), auto);
            manual = manual + 1;
        }

        if index % 2 != 0 {
            car = car_factory(colors[index-1].to_string(), engine, roof, miles);
            // ADD <K, V> pair to hash map
            orders.insert("Used".to_string(), used_cars);
            used_cars = used_cars + 1;
        } else { 
            car = car_factory(colors[index-1].to_string(), engine, roof, 0);
            // ADD <K, V> pair to hash map
            orders.insert("New".to_string(), new_cars);
            new_cars = new_cars + 1;
        }

        // Display car order details by roof type and age of car
        if car.roof && car.age.1 > 0 {
            println!("{}: {}, {:?}, Closed roof, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1); 
        } else if car.roof {
            println!("{}: {}, {:?}, Closed roof, {}", order, car.age.0, car.motor, car.color); 
        } else if car.age.1 > 0 {
            println!("{}: {}, {:?}, Convertible, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1); 
        } else {
            println!("{}: {}, {:?}, Convertible, {}", order, car.age.0, car.motor, car.color); 
        }

        order += 1;
        miles += 1000;

        if index < 4 {
            index += 1;
        } else {
            index = 1;
        }
    }
    println!("\nCar orders: {:?}", orders);
}
