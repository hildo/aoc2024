use crate::helpers;
use once_cell::sync::Lazy;
use regex::Regex;


fn load_lists(input_file_name: &str) -> (Vec<u64>, Vec<u64>) {
    static VALUES_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)\s*(\d+)").unwrap());

    let mut first_list: Vec<u64> = Vec::new();
    let mut second_list: Vec<u64> = Vec::new();

    if let Ok(lines) = helpers::read_lines(input_file_name) {
        for line in lines.flatten() {
            for c in VALUES_RE.captures_iter(&line) {
                let (_, [number1, number2]) = c.extract();
                match number1.parse() {
                    Ok(number) => first_list.push(number),
                    Err(_) => println!("Unable to parse {}", number1)
                };
                match number2.parse() {
                    Ok(number) => second_list.push(number),
                    Err(_) => println!("Unable to parse {}", number2)
                };
            }
        }
    }
    (first_list, second_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let (mut first_list, mut second_list) = load_lists("./src/resources/day01_simple.txt");
        first_list.sort();
        second_list.sort();

        let pair_list = first_list.iter().zip(second_list.iter());
        let diffs = pair_list.enumerate().map(|(_i, (v1, v2))| v1.abs_diff(*v2));
        let sum = diffs.fold(0, |acc, diff| acc + diff);
        assert_eq!(sum, 11);
    }

    #[test]
    fn test() {
        let (mut first_list, mut second_list) = load_lists("./src/resources/day01_input.txt");

        first_list.sort();
        second_list.sort();

        let diffs = first_list.iter().zip(second_list.iter()).enumerate()
            .map(|(_i, (v1, v2))| v1.abs_diff(*v2));
        let sum = diffs.fold(0, |acc, diff| acc + diff);
        println!("{}", sum);
        assert_eq!(sum, 1223326);
    }

}