fn main() {
    let input: &str = include_str!("./input_1.txt");
    let output: u32 = part_1(input);
    println!("{:?}", output);
}

fn part_1(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut first_digit: Option<char> = None;
        let mut second_digit: Option<char> = None;
        let mut start_pointer: usize = 0;
        let mut end_pointer: usize = line.len() - 1;

        while first_digit.is_none() || second_digit.is_none() {
            if line.chars().nth(start_pointer).unwrap().is_numeric() && first_digit.is_none() {
                first_digit = line.chars().nth(start_pointer);
            }
            if line.chars().nth(end_pointer).unwrap().is_numeric() && second_digit.is_none() {
                second_digit = line.chars().nth(end_pointer);
            }
            if first_digit.is_none() {
                start_pointer += 1;
            }
            if second_digit.is_none() {
                end_pointer -= 1;
            }
        }

        let combined_number: String =
            first_digit.unwrap().to_string() + &second_digit.unwrap().to_string();
        sum += combined_number.parse::<u32>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {

    use crate::part_1;

    #[test]
    fn test_1() {
        let input: &str = include_str!("./input_1.txt");
        let result: u32 = part_1(input);
        assert_eq!(142, result);
    }
}
