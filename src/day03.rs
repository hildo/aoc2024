use once_cell::sync::Lazy;
use regex::Regex;

fn find_pairs(input: &str) -> Vec<(u32, u32)> {
    let mut return_value: Vec<(u32, u32)> = Vec::new();
    static MULS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    let filtered_list = filter_dont(input);
    for c in MULS_RE.captures_iter(&filtered_list) {
        let (_, [number1, number2]) = c.extract();
        return_value.push((number1.parse().unwrap(), number2.parse().unwrap()));
    }


    return_value
}

fn filter_dont(input: &str) -> String {
    if !input.contains("don't()") {
        return input.to_string();
    }

    let mut return_value = String::new();

    let values: Vec<&str> = input.split("don't()").collect();
    return_value.push_str(values[0]);

    let mut idx = 1;
    while idx < values.len() {
        let substr = values[idx];
        if substr.contains("do()") {
            let sub_values: Vec<&str> = substr.split("do()").collect();
            let mut subidx = 1;
            while subidx < sub_values.len() {
                return_value.push_str(sub_values[subidx]);
                subidx += 1;
            }
        }
        idx += 1;
    }

    return_value
}

fn do_sum(pairs: Vec<(u32, u32)>) -> u32 {
    pairs.iter().fold(0, |acc, (num1, num2)| acc + (num1 * num2))
}

#[cfg(test)]
mod tests {
    use crate::helpers;

    use super::*;

    #[test]
    fn part_one_simple() {
        let pairs = find_pairs("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let sum = do_sum(pairs);
        assert_eq!(161, sum);
    }

    #[test]
    fn part_one() {
        let mut pair_list: Vec<Vec<(u32, u32)>> = Vec::new();

        if let Ok(lines) = helpers::read_lines("./src/resources/day03_input.txt") {
            for line in lines.flatten() {
                pair_list.push(find_pairs(&line));
            }
        }
    
        let sum = pair_list.iter()
            .map(|pairs| do_sum(pairs.to_vec()))
            .fold(0, |acc, value| acc + value);
        println!("{}", sum);
    }

    #[test]
    fn part_two_filter_dont() {
        let filtered_value = filter_dont("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!("xmul(2,4)&mul[3,7]!^?mul(8,5))", filtered_value);
    }

    #[test]
    fn part_two_filter_dont_long() {
        let filtered_value = filter_dont("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!("xmul(2,4)&mul[3,7]!^?mul(8,5))xmul(2,4)&mul[3,7]!^?mul(8,5))", filtered_value);
    }

    #[test]
    fn part_two_simple() {
        let pairs = find_pairs("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        let sum = do_sum(pairs);
        assert_eq!(48, sum);
    }

    #[test]
    fn part_two() {
        let mut mega_value = String::new();

        if let Ok(lines) = helpers::read_lines("./src/resources/day03_input.txt") {
            for line in lines.flatten() {
                mega_value.push_str(&line);
            }
        }
    
        let pairs = find_pairs(&mega_value);
        println!("{}", pairs.len());
        let sum = do_sum(pairs);
        println!("{}", sum);
        // not 72700613, 10046858 (too low), 43983129
        // answer is 100189366
    }

}