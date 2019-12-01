/// Calculates the fuel needed for the given mass
pub fn fuel(module_mass: i32) -> i32 {
    (module_mass / 3) - 2
}