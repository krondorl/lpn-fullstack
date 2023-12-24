// Made by Adam Burucs in 2023.
//
// License:
// Attribution-NonCommercial-NoDerivatives 4.0 International
// (CC BY-NC-ND 4.0)

pub fn sum_recursive(raw_number: u32) -> u32 {
    if raw_number < 10 {
        return raw_number;
    }
    sum_recursive(sum_numbers(raw_number))
}

pub fn sum_numbers(n: u32) -> u32 {
    let mut sum: u32 = 0;
    let binding = n.to_string();
    let numbers_split = binding.split("");
    for string_number in numbers_split {
        match string_number.parse::<u32>() {
            Ok(val) => sum += val,
            Err(e) => println!("Error when casting to integer: {e}"),
        }
    }
    sum
}
