extern crate regex;

use regex::Regex;

pub fn part1(inp: String) {
    let parse_result = read_input(&inp);

    let mut error_rate = 0;

    for ticket in parse_result.nearby_tickets {
        let invalid_values = find_invalid_values(&ticket, &parse_result.rules);
        error_rate += invalid_values.iter().fold(0, |acc, cur| acc + cur);
    }

    println!("Error rate: {}", error_rate);
}

pub fn part2(_inp: String) {}

fn find_invalid_values(values: &Vec<usize>, rules: &Vec<Rule>) -> Vec<usize> {
    values
        .iter()
        .filter(|value| !is_valid(&value, &rules))
        .map(|x| x.clone())
        .collect()
}

fn is_valid(value: &usize, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        let valid = rule.is_in_range(value);
        if valid {
            return true;
        }
    }
    false
}

fn read_input(inp: &String) -> ParseResult {
    let sections: Vec<&str> = inp.split("\n\n").filter(|line| line.len() > 0).collect();

    let rules = parse_rules_section(sections[0]);
    let your_ticket = parse_ticket_section(sections[1]).pop().unwrap();
    let nearby_tickets = parse_ticket_section(sections[2]);

    ParseResult{rules, your_ticket, nearby_tickets}
}

struct ParseResult {
    rules: Vec<Rule>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Rule {
    field: String,
    first_range: (usize, usize),
    second_range: (usize, usize),
}

impl Rule {
    pub fn is_in_range(&self, value: &usize) -> bool {
        self.first_range.0 <= *value && *value <= self.first_range.1
            || self.second_range.0 <= *value && *value <= self.second_range.1
    }
}

fn parse_rules_section(rule_section: &str) -> Vec<Rule> {
    rule_section
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| parse_rule(line))
        .collect()
}

fn parse_rule(rule_line: &str) -> Rule {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.*): (\\d+)-(\\d+) or (\\d+)-(\\d+)").unwrap();
    }

    let captures = RE.captures(rule_line).unwrap();

    let field: String = captures[1].to_string();

    let first_range = (
        captures[2].parse::<usize>().unwrap(),
        captures[3].parse::<usize>().unwrap(),
    );

    let second_range = (
        captures[4].parse::<usize>().unwrap(),
        captures[5].parse::<usize>().unwrap(),
    );

    Rule {
        field,
        first_range,
        second_range,
    }
}

fn parse_ticket_section(ticket_section: &str) -> Vec<Vec<usize>> {
    ticket_section
        .split("\n")
        .filter(|line| line.len() > 0)
        .filter(|line| !line.contains("ticket"))
        .map(|line| parse_ticket(line))
        .collect()
}

fn parse_ticket(ticket_line: &str) -> Vec<usize> {
    ticket_line
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static RULES_SECTION: &str = r#"
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50
"#;

    #[test]
    pub fn find_invalid_values_valid_value_returns_empty_vec() {
        let rules = parse_rules_section(RULES_SECTION);
        let ticket_values = vec![7, 3, 47];

        let invalid = find_invalid_values(&ticket_values, &rules);

        assert_eq!(invalid, vec![]);
    }

    #[test]
    pub fn find_invalid_values_invalid_value_returns_vec_with_value() {
        let rules = parse_rules_section(RULES_SECTION);
        let ticket_values = vec![40, 4, 50];

        let invalid = find_invalid_values(&ticket_values, &rules);

        assert_eq!(invalid, vec![4]);
    }
}
