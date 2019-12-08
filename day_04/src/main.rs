mod passwords;

fn main() {
    let start = 168630;
    let end = 718098;
    let valid = passwords::valid_inputs(start..end);
    println!("Total valid numbers: {}", valid);
}
