use std::collections::HashMap;

pub fn part1(inp: String) {
    let line = read_input(&inp);
    let sequence = calculate_sequence(line, 2020);
    println!("Number: {}", sequence.last().unwrap());
}

pub fn part2(inp: String) {
    let line = read_input(&inp);
    let sequence = calculate_sequence(line, 30_000_000);
    println!("Number: {}", sequence.last().unwrap());
}

fn read_input(inp: &String) -> &str {
    inp.split("\n").filter(|line| line.len() > 0).next().unwrap()
}

fn calculate_sequence(line: &str, limit: usize) -> Vec<usize> {
    let mut sequence: Vec<usize> = line
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // keys are numbers, values are last seen indices in sequence
    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    sequence.iter().enumerate().for_each(|(index, number)| {
        last_seen.insert(*number, index); // TODO don't insert last num
    });

    for index in sequence.len()..limit {
        let current_number = sequence[index - 1];
        let maybe_last_index = last_seen.get(&current_number);
        match maybe_last_index {
            Some(last_index) => {
                let age = index - last_index - 1;
                sequence.push(age);
            }
            None => {
                // never seen before
                sequence.push(0);
            }
        }
        last_seen.insert(current_number, index - 1);
    }

    sequence
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_value_with_mask() {}
}
