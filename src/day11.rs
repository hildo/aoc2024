fn blink_transform(input: &u128) -> Vec<u128> {
    if *input == 0 {
        return vec![1];
    }
    static TEN:u128 = 10;

    let digit_count= input.checked_ilog10().unwrap_or(0) + 1;
    let modulo = digit_count % 2;
    if modulo == 0 {
        let indice = digit_count / 2;
        let divisor = TEN.pow(indice as u32 );

        let left_num = input / divisor;
        let right_num = input - (left_num * divisor);

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

    #[test]
    fn check_part_two_75_blinks() {
        let mut stones = vec![7725, 185, 2, 132869, 0, 1840437, 62, 26310];
        for x in 0..75 {
            stones = blink(stones);
        }
        println!("{}", stones.len());
        // assert_eq!(stones.len(), 233050);
    }

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