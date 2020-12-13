pub fn part1(inp: String) {
    let (current_time, bus_ids) = read_input(&inp);

    let (bus_id, waiting_time) = bus_baun(current_time, &bus_ids);
    println!(
        "Bus ID: {}, waiting time: {}, result: {}",
        bus_id,
        waiting_time,
        bus_id * waiting_time
    );
}

fn bus_baun(current_time: usize, bus_ids: &Vec<usize>) -> (usize, usize) {
    bus_ids
        .iter()
        .map(|id| (*id, calc_waiting_time(current_time, *id)))
        .fold((0, usize::MAX), |acc, cur| {
            if cur.1 < acc.1 {
                (cur.0, cur.1)
            } else {
                acc
            }
        })
}

fn calc_waiting_time(current_time: usize, bus_interval: usize) -> usize {
    let past_arrivals = current_time / bus_interval; // 939 / 59 = 15.9 = 15
    let time_of_last_arrival = past_arrivals * bus_interval; // 15 * 59 = 885
    let time_of_next_arrival = time_of_last_arrival + bus_interval; // 885 + 59 = 944
    let time_until_next_arrival = time_of_next_arrival - current_time; // 944 - 939 = 5
    time_until_next_arrival // quick math :D
}

pub fn part2(_inp: String) {
    println!("Something something Chinese remainder theorem. We gave up. ¯\\_(ツ)_/¯");
}

fn read_input(inp: &str) -> (usize, Vec<usize>) {
    let lines: Vec<&str> = inp.split("\n").filter(|line| line.len() > 0).collect();

    let current_time = lines[0].parse::<usize>().unwrap();

    let bus_ids: Vec<usize> = lines[1]
        .split(",")
        .filter(|entry| *entry != "x")
        .map(|id| id.parse::<usize>().unwrap())
        .collect();

    (current_time, bus_ids)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn read_input_part1_sample() {
        let input = "939\n7,13,x,x,59,x,31,19";

        let result = read_input(&input);

        assert_eq!(result, (939, vec![7, 13, 59, 31, 19]));
    }

    #[test]
    pub fn bus_baun_part1_sample() {
        let input_str = "939\n7,13,x,x,59,x,31,19";
        let (current_time, bus_ids) = read_input(input_str);

        let result = bus_baun(current_time, &bus_ids);

        assert_eq!(result, (59, 5));
    }

    #[test]
    pub fn calc_waiting_time_bus_59() {
        let result = calc_waiting_time(939, 59);

        assert_eq!(result, 5);
    }
}
