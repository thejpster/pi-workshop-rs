extern crate sensehat;

fn main() {
    let mut hat = sensehat::SenseHat::new().expect("Couldn't find Sense Hat");
    let temp = hat.get_temperature_from_humidity();
    println!("Hello, world!");
}
