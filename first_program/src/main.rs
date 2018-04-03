
// Create my first struct
struct Car<'a>{
    model: &'a str,
    year: u16,
    mpg: f32,
    tank_size: f32,
}

// Just checking out how the methods in rust are
// Probably won't use too much, I just wanted to see :)
impl<'a>  Car<'a> {
fn calculate_max_miles(mpg:f32,tank_size:f32) -> f32 {
    return mpg * tank_size;
    }
}


// Main Function
fn main() {

    // Create our variables
    let model = "Chevy Malibu";
    let year = 1998;
    let mpg: f32 = 23.0;
    let tank_size: f32 = 15.3;
    // Create my struct
    let my_first_car = Car{model, year, mpg,tank_size};


    // Print my max miles per tank
    println!("The Max miles in one tank for {:?} with the year {:?} is {:?}"
             ,my_first_car.model,my_first_car.year,
             Car::calculate_max_miles(my_first_car.mpg,my_first_car.tank_size));


}
