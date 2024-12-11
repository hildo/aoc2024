use std::collections::HashMap;

fn blink_transform_map(input: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut return_value = HashMap::with_capacity(input.len());

    for (&key, &count) in input {
        match key {
            0 => *return_value.entry(1).or_default() += count,
            _ => {
                let digits = key.ilog10() + 1;
                if digits % 2 == 0 {
                    *return_value.entry(key % 10u64.pow(digits / 2)).or_default() += count;
                    *return_value.entry(key / 10u64.pow(digits / 2)).or_default() += count;
                } else {
                    *return_value.entry(key * 2024).or_default() += count;
                }
            }
        }
    }

    return_value
}

fn load_value_map(input: Vec<u64>) -> HashMap<u64, usize> {
    let mut return_value = HashMap::with_capacity(input.len());
    for value in input {
        return_value.insert(value,1);
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample_6_blinks() {
        let mut stones = load_value_map(vec![125, 17]);
        for x in 0..6 {
            stones = blink_transform_map(&stones);
        }
        let total_stones: usize = stones.values().sum();
        assert_eq!(total_stones, 22);
    }

    #[test]
    fn check_sample_25_blinks() {
        let mut stones = load_value_map(vec![125, 17]);
        for x in 0..25 {
            stones = blink_transform_map(&stones);
        }
        let total_stones: usize = stones.values().sum();
        assert_eq!(total_stones, 55312);
    }

    #[test]
    fn check_part_one_25_blinks() {
        let mut stones = load_value_map(vec![7725, 185, 2, 132869, 0, 1840437, 62, 26310]);
        for x in 0..25 {
            stones = blink_transform_map(&stones);
        }
        let total_stones: usize = stones.values().sum();
        assert_eq!(total_stones, 233050);
    }

    #[test]
    fn check_part_two_75_blinks() {
        let mut stones = load_value_map(vec![7725, 185, 2, 132869, 0, 1840437, 62, 26310]);
        for x in 0..75 {
            stones = blink_transform_map(&stones);
        }
        let total_stones: usize = stones.values().sum();
        assert_eq!(total_stones, 276661131175807);
    }

}