use crate::helpers;

fn load_chars(file_name: &str) -> Vec<String> {
    let mut return_value: Vec<String> = Vec::new();
    if let Ok(lines) = helpers::read_lines("./src/resources/day03_input.txt") {
        for line in lines.flatten() {
            return_value.push(line);
        }
    
    }
    return_value
}

fn test_forward(input: &Vec<String>, coord: (usize, usize)) -> bool {
    let row = input.get(coord.0).unwrap();
    (coord.1 + 3 < row.len())
        && row.chars().nth(coord.1) == Some('X')
        && row.chars().nth(coord.1 + 1) == Some('M')
        && row.chars().nth(coord.1 + 2) == Some('A')
        && row.chars().nth(coord.1 + 3) == Some('S')
}

fn test_backward(input: &Vec<String>, coord: (usize, usize)) -> bool {
    let row = input.get(coord.0).unwrap();
    (coord.1 > 3 )
        && row.chars().nth(coord.1) == Some('X')
        && row.chars().nth(coord.1 - 1) == Some('M')
        && row.chars().nth(coord.1 - 2) == Some('A')
        && row.chars().nth(coord.1 - 3) == Some('S')
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_forward() {
        let mut my_vec = Vec::new();
        my_vec.push("XMASASAMXFH".to_string());

        assert_eq!(true, test_forward(&my_vec, (0, 0)));
        assert_eq!(false, test_forward(&my_vec, (0, 3)));
        assert_eq!(false, test_forward(&my_vec, (0, 9)));
        assert_eq!(true, test_backward(&my_vec, (0, 7)));
    }
}