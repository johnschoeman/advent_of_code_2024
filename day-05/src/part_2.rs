use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

const FILE_PATH: &str = "./input1.txt";

type Page<'a> = &'a str;
type OrderingRule<'a> = (Page<'a>, Page<'a>);
type PageList<'a> = Vec<Page<'a>>;
type OrderingRules<'a> = HashMap<Page<'a>, Vec<Page<'a>>>;

pub fn run() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(FILE_PATH)?;
    match process(&contents) {
        Ok(result) => Ok(result.to_string()),
        Err(e) => Err(e.into()),
    }
}

fn process(input: &str) -> Result<usize, String> {
    match parse(input) {
        Ok((_remaining, results)) => {
            let (ordering_rules, page_lists) = results;
            let ordering_rules_map = build_ordering_rules_map(&ordering_rules);

            let sum = page_lists
                .iter()
                .filter_map(|page_list| {
                    if !is_ordering_valid(&ordering_rules_map, page_list) {
                        Some(sort(&ordering_rules_map, page_list))
                            .map(|page_list| middle_element(&page_list))
                    } else {
                        None
                    }
                })
                .sum::<usize>();

            Ok(sum)
        }
        Err(_err) => Err("parsing failed".to_string()),
    }
}

fn sort<'a>(ordering_rules: &'a OrderingRules, page_list: &'a PageList) -> PageList<'a> {
    let mut list = page_list.clone();
    list.sort_by(|a, b| compare(ordering_rules, a, b));
    list
}

fn compare(ordering_rules: &OrderingRules, a: Page, b: Page) -> Ordering {
    let default_lower_pages = vec![];
    let lower_pages = ordering_rules.get(a).unwrap_or(&default_lower_pages);
    if lower_pages.contains(&b) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn middle_element<'a>(list: &'a Vec<&'a str>) -> usize {
    let idx = list.len() / 2;
    list[idx].parse::<usize>().unwrap_or_default()
}

fn build_ordering_rules_map<'a>(ordering_rules: &'a Vec<OrderingRule<'a>>) -> OrderingRules<'a> {
    let mut map: OrderingRules<'a> = HashMap::new();
    for (lower, upper) in ordering_rules {
        map.entry(upper).or_insert(vec![]).push(lower);
    }
    map
}

fn is_ordering_valid(ordering_rules: &OrderingRules, page_list: &PageList) -> bool {
    page_list
        .iter()
        .enumerate()
        .fold(true, |acc, (index, &page)| {
            if !acc {
                return false;
            }
            let next_pages = &page_list[(index + 1)..];
            let default_lower_pages = vec![];
            let lower_pages = ordering_rules.get(page).unwrap_or(&default_lower_pages);
            let is_not_ordered = next_pages
                .iter()
                .any(|later_page| lower_pages.contains(later_page));
            !is_not_ordered
        })
}

fn parse(input: &str) -> IResult<&str, (Vec<OrderingRule>, Vec<PageList>)> {
    let (next, ordering_rules) = many1(terminated(ordering_rule, newline))(input)?;
    let (next, _) = newline(next)?;
    let (next, page_lists) = many1(terminated(page_list, newline))(next)?;

    Ok((next, (ordering_rules, page_lists)))
}

fn ordering_rule(input: &str) -> IResult<&str, OrderingRule> {
    separated_pair(digit1, tag("|"), digit1)(input)
}

fn page_list(input: &str) -> IResult<&str, PageList> {
    separated_list1(tag(","), digit1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let contents = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

        assert_eq!(123, process(contents)?);
        Ok(())
    }
}
