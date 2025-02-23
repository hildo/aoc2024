use crate::helpers;
use once_cell::sync::Lazy;
use regex::Regex;

/*
    PageRule.  Before must come vefore after
 */
struct PageRule {
    before_page_num: u8,
    after_page_num: u8
}

fn load_input(input_file_name: &str) -> (Vec<PageRule>, Vec<Vec<u8>>) {
    static RULES_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<first>\d+)\|(?<second>\d+)").unwrap());

    let mut rules: Vec<PageRule> = Vec::new();
    let mut pages: Vec<Vec<u8>> = Vec::new();

    if let Ok(lines) = helpers::read_lines(input_file_name) {
        for line in lines.flatten() {
            // If there is a "|", this is a rule
            // If there are commas, it will be a list of pages
            println!("{}", line);
            if RULES_RE.is_match(&line) {
                let capture = RULES_RE.captures(&line).unwrap();
                
                rules.push(PageRule{
                    before_page_num: capture["first"].parse().unwrap(),
                    after_page_num: capture["second"].parse().unwrap()
                });
            } else if !line.is_empty() {
                let mut these_pages: Vec<u8> = Vec::new();
                line.split(",").for_each(|strvalue| these_pages.push(strvalue.parse().unwrap())); 
                pages.push(these_pages);
            }

        }
    }

    return (rules, pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let (rules, pages) = load_input("./src/resources/day05_simple.txt");
    }
}