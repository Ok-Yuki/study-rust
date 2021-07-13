#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
// TO DO: Replace the "mileage" field from the previous exercise with an "age" field
// TO DO" The "age" field should hold tuple value of two fields: String, u32
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
    // TO DO: Replace the "mileage" field from the previous exercise with an "age" field
    // TO DO" The "age" field calls the "car_quality" function with the "miles" input argument 
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
    let colors = ["Blue", "Green", "Red", "Silver"];

    let (mut index, mut order) = (1, 1);

    let mut car: Car;
    let mut miles = 1000;
    let mut roof = true;
    let mut engine: Transmission;

    while order < 12 {
        engine = Transmission::Manual;

        if index % 2 != 0 {
            car = car_factory(colors[index - 1].to_string(), engine, roof, miles);
        } else {
            car = car_factory(colors[index - 1].to_string(), engine, roof, 0);
        }

        println!("{}: {}, Closed roof, {:?}, {}, {} miles", order, car.age.0, car.motor, car.color, car.age.1);


        order += 1;
        miles += 1000;

        if index < 4 {
            index += 1;
        } else {
            index = 1;
        }
    }
}
