/// Calculates the fuel needed for the given mass
pub fn fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

/// Calculates the fuel needed for the given module, including fuel needed for it's own fuel
pub fn total_fuel(module_mass: i32) -> i32 {
    let mut mass_fuel: i32 = self::fuel(module_mass);
    // If the fuel is > 0, add the fuel for the remainder recursively
    if mass_fuel > 0 {
        mass_fuel += self::total_fuel(mass_fuel);
        return mass_fuel;
    }
    // Else, return 0 explicitly, so we don't end up returning a negative value and getting the
    // wrong result
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
    }

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(12), 2);
        assert_eq!(total_fuel(1969), 966);
        assert_eq!(total_fuel(100756), 50346);
    }
}