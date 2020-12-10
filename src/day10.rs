use std::collections::HashMap;
use std::collections::HashSet;

pub fn part1(inp: String) {
    let mut numbers = read_numbers(&inp);
    let (diff_of_1, diff_of_3) = find_diffs(&mut numbers);
    let mult = diff_of_1 * diff_of_3;
    println!("1s: {} / 3s: {} / mult: {}", diff_of_1, diff_of_3, mult);
}

fn find_diffs(numbers: &mut Vec<usize>) -> (usize, usize) {
    numbers.push(0); // outlet is 0
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3); // diff to device is always 3

    let mut diff_of_1 = 0;
    let mut diff_of_3 = 0;

    //    [(0), 2, 3, 4, 6, (last+3)]
    // -> [   2, 1, 1, 2 ]

    for (index, number) in numbers.iter().enumerate().skip(1) {
        let diff = number - numbers[index - 1];
        if diff == 1 {
            diff_of_1 += 1;
        }
        if diff == 3 {
            diff_of_3 += 1;
        }
    }

    (diff_of_1, diff_of_3)
}

pub fn part2(inp: String) {
    let numbers = read_numbers(&inp);


    // just to trigger the debug output
    let input = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
    let result = calc_next_adaptors(&input);
    // just to trigger the debug output


    // println!("TODO: {:?}", numbers);
}

fn calc_next_adaptors(adaptors: &Vec<usize>) -> HashMap<usize, Vec<usize>> {
    let mut next_adaptors = HashMap::<usize, Vec<usize>>::new();

    for (index, _) in adaptors.iter().enumerate() {
        let mut next_for_this: Vec<usize> = vec![];
        for diff in 1..=3 {
            let ancestor = adaptors.get(index + diff); // FIXME BUG BUG BUG
            match ancestor {
                Some(next_adaptor) => next_for_this.push(*next_adaptor),
                None => (),
            }
        }
        next_adaptors.insert(index, next_for_this);
    }

    println!("{:?}", next_adaptors);

    next_adaptors
}

fn read_numbers(inp: &str) -> Vec<usize> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn find_diffs_part1_small_example() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let (diff_of_1, diff_of_3) = find_diffs(&mut input);

        assert_eq!(diff_of_1, 7);
        assert_eq!(diff_of_3, 5);
    }

    #[test]
    pub fn find_diffs_part1_large_example() {
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        let (diff_of_1, diff_of_3) = find_diffs(&mut input);

        assert_eq!(diff_of_1, 22);
        assert_eq!(diff_of_3, 10);
    }

    #[test]
    pub fn calc_next_adaptors_small_input() {
        let input = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
        let next_for_0: Vec<usize> = vec![1];
        let next_for_1: Vec<usize> = vec![4];
        let next_for_4: Vec<usize> = vec![5, 6, 7];

        let result = calc_next_adaptors(&input);

        assert_eq!(*result.get(&0).unwrap(), next_for_0);
        assert_eq!(*result.get(&1).unwrap(), next_for_1);
        assert_eq!(*result.get(&2).unwrap(), next_for_4);
    }
}
