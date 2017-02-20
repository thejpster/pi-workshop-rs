extern crate sensehat;

fn main() {
    let mut hat = sensehat::SenseHat::new().expect("Couldn't find Sense Hat");
    let temp = hat.get_temperature_from_humidity().expect("Couldn't read temp");
    println!("Hello, world! It's {}", temp);
}
