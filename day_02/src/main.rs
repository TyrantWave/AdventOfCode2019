use std::fs;

mod opcode;

fn make_inputs(file: &str) -> Vec<i32> {
    file.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("src/inputs")?;
    let mut numbers = make_inputs(&file);

    numbers[1] = 12;
    numbers[2] = 2;
    let output = opcode::opcodes(&mut numbers);
    println!("Part 1: {}", output);

    let required_output = 19_690_720;
    for noun in 1..numbers.len() as i32 {
        for verb in 1..numbers.len() as i32 {
            numbers = make_inputs(&file);
            numbers[1] = noun;
            numbers[2] = verb;
            if opcode::opcodes(&mut numbers) == required_output {
                println!(
                    "Part 2: Required noun: {}, verb: {}, mul: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return Ok(());
            }
        }
    }

    Ok(())
}
