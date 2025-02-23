use crate::helpers;
use once_cell::sync::Lazy;
use regex::Regex;

/*
    PageRule.  Before must come vefore after
 */
struct PageRule {
    before_page_num: u32,
    after_page_num: u32
}

fn load_input(input_file_name: &str) -> (Vec<PageRule>, Vec<Vec<u32>>) {
    static RULES_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<first>\d+)\|(?<second>\d+)").unwrap());

    let mut rules: Vec<PageRule> = Vec::new();
    let mut pages: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = helpers::read_lines(input_file_name) {
        for line in lines.flatten() {
            // If there is a "|", this is a rule
            // If there are commas, it will be a list of pages
            if RULES_RE.is_match(&line) {
                let capture = RULES_RE.captures(&line).unwrap();
                
                rules.push(PageRule{
                    before_page_num: capture["first"].parse().unwrap(),
                    after_page_num: capture["second"].parse().unwrap()
                });
            } else if !line.is_empty() {
                let mut these_pages: Vec<u32> = Vec::new();
                line.split(",").for_each(|strvalue| these_pages.push(strvalue.parse().unwrap())); 
                pages.push(these_pages);
            }
        }
    }

    return (rules, pages)
}

fn are_pages_ordered(pages: &Vec<u32>, rules: &Vec<PageRule>) -> bool {
    rules.iter().map(|rule| test_pages(&pages, &rule)).all(|rule_matched| rule_matched)
}

fn test_pages(pages: &Vec<u32>, rule: &PageRule) -> bool {
    let before_pos = pages.iter().position(|&x| x == rule.before_page_num);
    let after_pos = pages.iter().position(|&x| x == rule.after_page_num);
    
    match (before_pos, after_pos) {
        (Some(before), Some(after)) => before < after,
        _ => true
    }
}

fn middle_entry(pages: &Vec<u32>) -> u32 {
    let middle_idx = pages.len() / 2;
    pages[middle_idx]
}

fn correct_order(pages: &Vec<u32>, rules: &Vec<PageRule>) -> Vec<u32> {
    let mut return_value = pages.clone();
    let mut rule_idx = 0;
    while rule_idx < rules.len() {
        let rule = &rules[rule_idx];
        if test_pages(&return_value, rule) {
            rule_idx += 1;
            continue;
        }
        let current_before_idx = return_value.iter().position(|&x| x == rule.before_page_num);
        let current_after_idx = return_value.iter().position(|&x| x == rule.after_page_num);
        match (current_before_idx, current_after_idx) {
            (Some(before), Some(after)) => {
                return_value.remove(before);
                return_value.insert(after, rule.before_page_num);
                rule_idx = 0;
                continue;
            }
            _ => {
                // nothing
            }
        }
        rule_idx += 1;
    }
    return_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let (rules, all_pages) = load_input("./src/resources/day05_input.txt");
        let entry_sum: u32 = all_pages.iter()
            .filter(|pages| !are_pages_ordered(pages, &rules))
            .map(|pages| correct_order(pages, &rules))
            .map(|pages| middle_entry(&pages))
            .sum();
        assert_eq!(4598, entry_sum);
    }

    #[test]
    fn test_correct_order() {
        let (rules, all_pages) = load_input("./src/resources/day05_simple.txt");
        assert_eq!(vec![97,75,47,61,53], correct_order(&all_pages[3], &rules));
        assert_eq!(vec![61,29,13], correct_order(&all_pages[4], &rules));
        assert_eq!(vec![97,75,47,29,13], correct_order(&all_pages[5], &rules));

        let entry_sum: u32 = all_pages.iter()
            .filter(|pages| !are_pages_ordered(pages, &rules))
            .map(|pages| correct_order(pages, &rules))
            .map(|pages| middle_entry(&pages))
            .sum();
        assert_eq!(123, entry_sum);

    }

    #[test]
    fn test_part_one() {
        let (rules, all_pages) = load_input("./src/resources/day05_input.txt");
        let entry_sum: u32 = all_pages.iter()
            .filter(|pages| are_pages_ordered(pages, &rules))
            .map(|pages| middle_entry(pages))
            .sum();
        assert_eq!(5452, entry_sum);
    }

    #[test]
    fn test_simple() {
        let (rules, all_pages) = load_input("./src/resources/day05_simple.txt");
        assert!(are_pages_ordered(&all_pages[0], &rules));
        assert_eq!(61, middle_entry(&all_pages[0]));
        assert!(are_pages_ordered(&all_pages[1], &rules));
        assert_eq!(53, middle_entry(&all_pages[1]));
        assert!(are_pages_ordered(&all_pages[2], &rules));
        assert_eq!(29, middle_entry(&all_pages[2]));
        assert!(!are_pages_ordered(&all_pages[3], &rules));
        assert!(!are_pages_ordered(&all_pages[4], &rules));
        assert!(!are_pages_ordered(&all_pages[5], &rules));

        let entry_sum: u32 = all_pages.iter()
            .filter(|pages| are_pages_ordered(pages, &rules))
            .map(|pages| middle_entry(pages))
            .sum();
        assert_eq!(143, entry_sum);
    }

    #[test]
    fn test_test_pages() {
        let pages = vec![13, 44, 23, 67];
        assert!(test_pages(&pages, &PageRule{ before_page_num: 44, after_page_num: 23}));
        assert!(test_pages(&pages, &PageRule{ before_page_num: 13, after_page_num: 23}));
        assert!(test_pages(&pages, &PageRule{ before_page_num: 23, after_page_num: 67}));
        assert!(!test_pages(&pages, &PageRule{ before_page_num: 23, after_page_num: 44}));
        assert!(test_pages(&pages, &PageRule{ before_page_num: 23, after_page_num: 99}));
    }
}