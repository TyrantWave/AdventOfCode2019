pub fn opcodes(numbers: &mut Vec<i32>) -> i32 {
    let mut idx: usize = 0;

    loop {
        if numbers[idx] == 99 {
            break;
        }

        let first_idx = numbers[idx + 1] as usize;
        let second_idx = numbers[idx + 2] as usize;
        let third_idx = numbers[idx + 3] as usize;

        match numbers[idx] {
            1 => {
                numbers[third_idx] = numbers[first_idx] + numbers[second_idx];
            }
            2 => {
                numbers[third_idx] = numbers[first_idx] * numbers[second_idx];
            }
            _ => panic!("Invalid opcode! idx: {}, val: {}", idx, numbers[idx]),
        }
        idx += 4;
    }

    numbers[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_opcodes() {
        let mut num1 = vec![1, 0, 0, 0, 99];
        assert_eq!(opcodes(&mut num1), 2);
        assert_eq!(num1, vec![2, 0, 0, 0, 99]);

        let mut num2 = vec![2, 3, 0, 3, 99];
        assert_eq!(opcodes(&mut num2), 2);
        assert_eq!(num2, vec![2, 3, 0, 6, 99]);

        let mut num3 = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(opcodes(&mut num3), 2);
        assert_eq!(num3, vec![2, 4, 4, 5, 99, 9801]);

        let mut num4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(opcodes(&mut num4), 30);
        assert_eq!(num4, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
