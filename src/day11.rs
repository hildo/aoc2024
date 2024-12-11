fn blink_transform(input: &u128) -> Vec<u128> {
    if *input == 0 {
        return vec![1];
    }
    let digits: Vec<_> = input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let modulo = digits.len() % 2;
    if modulo == 0 {
        // Even number of digits.  Split down the middle and form two number
        let idx = digits.len() / 2;
        let (left, right) = digits.split_at(idx);
        let mut str_value = String::new();
        for digit in left.iter().map(|d| d.to_string())  {
            str_value.push_str(&digit);
        }
        let left_num: u128 = str_value.parse().unwrap();

        str_value = String::new();
        for digit in right.iter().map(|d| d.to_string())  {
            str_value.push_str(&digit);
        }
        let right_num: u128 = str_value.parse().unwrap();

        return vec![left_num, right_num];
   } else {
        return vec![input * 2024];
   }
}

fn blink(input: Vec<u128>) -> Vec<u128> {
    input.iter()
        .flat_map(|value| blink_transform(value))
        .collect()
        
}

#[cfg(test)]
mod tests {
    use crate::helpers;

    use super::*;

    #[test]
    fn check_sample() {
        let blinked = blink(vec![0, 1, 10, 99, 999]);
        assert_eq!(blink(vec![0, 1, 10, 99, 999]), vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn check_sample_6_blinks() {
        let mut stones = vec![125, 17];
        for x in 0..6 {
            stones = blink(stones);
        }
        assert_eq!(stones.len(), 22);
        assert_eq!(stones, vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]);
    }

    #[test]
    fn check_sample_25_blinks() {
        let mut stones = vec![125, 17];
        for x in 0..25 {
            stones = blink(stones);
        }
        assert_eq!(stones.len(), 55312);
    }

    #[test]
    fn check_part_one_25_blinks() {
        let mut stones = vec![7725, 185, 2, 132869, 0, 1840437, 62, 26310];
        for x in 0..25 {
            stones = blink(stones);
        }
        assert_eq!(stones.len(), 233050);
    }

    // #[test]
    // fn check_part_two_75_blinks() {
    //     let mut stones = vec![7725, 185, 2, 132869, 0, 1840437, 62, 26310];
    //     for x in 0..75 {
    //         stones = blink(stones);
    //     }
    //     println!("{}", stones.len());
    //     // assert_eq!(stones.len(), 233050);
    // }

    #[test]
    fn test_blink_tranform() {
        assert_eq!(blink_transform(&0), vec![1]);
        assert_eq!(blink_transform(&1), vec![2024]);
        assert_eq!(blink_transform(&10), vec![1, 0]);
        assert_eq!(blink_transform(&1000), vec![10, 0]);
        assert_eq!(blink_transform(&99), vec![9, 9]);
        assert_eq!(blink_transform(&999), vec![2021976]);
    }
}