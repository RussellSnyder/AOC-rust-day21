use std::collections::HashSet;

#[derive(Debug)]
struct Bag {
    color: String,
    count: i16,
}

fn create_bag(color_1: &str, color_2: &str, count_str: &str) -> Bag {
    let mut color = color_1.to_owned();
    color.push_str(" ");
    color.push_str(color_2);

    let count = count_str.parse::<i16>().unwrap();

    Bag { count, color }
}

#[derive(Debug)]
struct Rule {
    color: String, // top level bag color
    contained_bags: Option<Vec<Bag>>,
}

pub fn part1(inp: String) {
    /*
    let lines = read_lines(
        r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#,
    );
    */
    let lines = read_lines(&inp);

    // TODO a hashmap instead of a Vec would be better for lookup :-(
    let rules: Vec<Rule> = lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| create_rule(line))
        .collect();

    println!("Rules: {:?}", rules);
    println!("# of rules: {:?}", rules.len());

    let shiny_gold_holding_colors = get_shiny_gold_holding_bag_colors(rules);

    println!("Colors of bags that can hold shiny gold: {:?}", shiny_gold_holding_colors);

    println!("# of bag colors that can hold shiny gold: {}", shiny_gold_holding_colors.len());
}

fn get_shiny_gold_holding_bag_colors(rules: Vec<Rule>) -> HashSet<String> {
    let mut shiny_gold_holding_colors = HashSet::<String>::new();

    for rule in rules.iter() {
        if can_contain_shiny_gold_bag(
            &rule.color,
            &rules,
            &mut shiny_gold_holding_colors,
        ) {
            shiny_gold_holding_colors.insert(rule.color.to_string());
        }
    }

    shiny_gold_holding_colors
}

fn can_contain_shiny_gold_bag(
    color: &String,
    rules: &Vec<Rule>,
    shiny_gold_holding_colors: &mut HashSet<String>,
) -> bool {
    if shiny_gold_holding_colors.contains(color) {
        return true;
    }

    // would be better if rules was a HashMap :-(
    let rule = rules.iter().find(|r| r.color == *color);
    if rule.is_none() {
        // we should rather panic…
        println!("No rule for color {}", color);
        return false;
    }

    match &rule.unwrap().contained_bags {
        None => {
            return false;
        }
        Some(bags) => {
            for inner_bag in bags.iter() {
                if is_shiny_gold_bag(inner_bag) {
                    return true;
                } else {
                    if can_contain_shiny_gold_bag(
                        &inner_bag.color,
                        rules,
                        shiny_gold_holding_colors,
                    ) {
                        return true;
                    }
                }
            }
            return false;
        }
    }
}

fn is_shiny_gold_bag(bag: &Bag) -> bool {
    bag.color == "shiny gold"
}

pub fn part2(inp: String) {
    let lines = read_lines(&inp);

    println!("{:?}", lines);
}

fn read_lines(inp: &str) -> Vec<&str> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>()
}

fn create_rule(line: &str) -> Rule {
    let groups = line.split(" bags contain ");

    let vec: Vec<&str> = groups.collect();

    let color = vec[0].to_owned();
    let contained_bags: Option<Vec<Bag>> = vec[1]
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.split(" ").collect::<Vec<&str>>())
        .map(|value| match value[0] {
            "no" => None,
            _ => Some(create_bag(value[1], value[2], value[0])),
        })
        .collect();

    Rule {
        color,
        contained_bags,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn get_shiny_gold_holding_bag_colors_single_empty_bag() {
        let rule = Rule {
            color: "testing green".to_owned(),
            contained_bags: None,
        };

        let bag_colors = get_shiny_gold_holding_bag_colors(vec![rule]);

        assert_eq!(bag_colors.len(), 0);
    }

    #[test]
    pub fn get_shiny_gold_holding_bag_colors_single_bag_holding_gold() {
        let shiny_bag = create_bag("shiny", "gold", "4");
        let rule = Rule {
            color: "testing green".to_owned(),
            contained_bags: Some(vec![shiny_bag]),
        };

        let bag_colors = get_shiny_gold_holding_bag_colors(vec![rule]);

        assert_eq!(bag_colors.len(), 1);
    }

    #[test]
    pub fn get_shiny_gold_holding_bag_colors_single_bag_holding_gold_and_other() {
        let shiny_bag = create_bag("shiny", "gold", "4");
        let other_bag = create_bag("bording", "grey", "1");
        let rule = Rule {
            color: "testing green".to_owned(),
            contained_bags: Some(vec![shiny_bag, other_bag]),
        };

        let bag_colors = get_shiny_gold_holding_bag_colors(vec![rule]);

        assert_eq!(bag_colors.len(), 1);
    }

    #[test]
    pub fn get_shiny_gold_holding_bag_colors_multiple_rules_one_direct_gold() {
        let shiny_bag = create_bag("shiny", "gold", "4");
        let other_bag = create_bag("bording", "grey", "1");
        let rule_no_gold = Rule {
            color: "raging red".to_owned(),
            contained_bags: None,
        };
        let rule_gold = Rule {
            color: "testing green".to_owned(),
            contained_bags: Some(vec![shiny_bag, other_bag]),
        };

        let bag_colors = get_shiny_gold_holding_bag_colors(vec![rule_no_gold, rule_gold]);

        assert_eq!(bag_colors.len(), 1);
    }

    #[test]
    pub fn get_shiny_gold_holding_bag_colors_multiple_rules_one_gold_nested_level_1() {
        let direct = Rule {
            color: "bright white".to_owned(),
            contained_bags: Some(vec![create_bag("shiny", "gold", "1")]),
        };
        let indirect_level_1 = Rule {
            color: "dark orange".to_owned(),
            contained_bags: Some(vec![create_bag("bright", "white", "4")]),
        };
        let rule_no_gold = Rule {
            color: "raging red".to_owned(),
            contained_bags: None,
        };

        let bag_colors =
            get_shiny_gold_holding_bag_colors(vec![direct, indirect_level_1, rule_no_gold]);

        assert_eq!(bag_colors.len(), 2);
    }

    #[test]
    pub fn get_shiny_gold_holding_bag_colors_multiple_rules_one_gold_nested_level_2() {
        let direct = Rule {
            color: "bright white".to_owned(),
            contained_bags: Some(vec![create_bag("shiny", "gold", "1")]),
        };
        let indirect_level_1 = Rule {
            color: "dark orange".to_owned(),
            contained_bags: Some(vec![create_bag("bright", "white", "4")]),
        };
        let indirect_level_2 = Rule {
            color: "goofy grey".to_owned(),
            contained_bags: Some(vec![create_bag("dark", "orange", "3")]),
        };
        let rule_no_gold = Rule {
            color: "raging red".to_owned(),
            contained_bags: None,
        };

        let bag_colors = get_shiny_gold_holding_bag_colors(vec![
            indirect_level_2,
            rule_no_gold,
            direct,
            indirect_level_1,
        ]);

        assert_eq!(bag_colors.len(), 3);
    }
}
