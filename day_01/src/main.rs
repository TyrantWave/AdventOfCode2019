use std::fs::File;
use std::io::{BufRead, BufReader};

mod fuel;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("src/inputs")?;
    let buffered = BufReader::new(file);
    let mut fuel_needed: i32 = 0;
    let mut total_modules: i32 = 0;

    for line in buffered.lines() {
        let mass = line?.parse::<i32>()?;
        let mass_fuel = fuel::total_fuel(mass);
        println!("Module mass: {}, fuel needed: {}", mass, mass_fuel);
        fuel_needed += mass_fuel;
        total_modules += 1;
    }
    println!("\nTotal modules given: {}", total_modules);
    println!("Fuel needed: {}", fuel_needed);

    Ok(())
}
