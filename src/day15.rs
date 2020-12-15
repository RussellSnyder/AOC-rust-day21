use std::collections::HashMap;

pub fn part1(inp: String) {
    let line: &str = inp.split("\n").filter(|line| line.len() > 0).next().unwrap();

    let mut sequence: Vec<usize> = line
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // keys are numbers, values are last seen indices in sequence
    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    sequence.iter().enumerate().for_each(|(index, number)| {
        last_seen.insert(*number, index); // TODO don't insert last num
    });

    for index in sequence.len()..2020 {
        let current_number = sequence[index - 1];
        let maybe_last_index = last_seen.get(&current_number);
        println!(
            "cur num: {} / maybe idx: {:?}",
            current_number, maybe_last_index
        );
        match maybe_last_index {
            Some(last_index) => {
                let age = index - last_index - 1;
                println!("age: {}", age);
                sequence.push(age);
            }
            None => {
                // never seen before
                sequence.push(0);
            }
        }
        println!("pushing num {} with index {}", current_number, index - 1);
        last_seen.insert(current_number, index - 1);
    }

    println!("{:?}", sequence);

    println!("Number: {}", sequence.last().unwrap());
}

pub fn part2(_inp: String) {
    //TODO
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_value_with_mask() {}
}
