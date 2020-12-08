use std::collections::HashSet;

pub fn part1(inp: String) {
    let instructions = read_instructions(&inp);
    let mut acc: i32 = 0;
    let mut pc: usize = 0;
    let mut seen_instructions = HashSet::<usize>::new();

    while !seen_instructions.contains(&pc) {
        seen_instructions.insert(pc);
        let instruction = instructions[pc];
        exec(&instruction, &mut pc, &mut acc);
    }

    println!("acc: {}", acc);
}

pub fn part2(inp: String) {}

// TODO: & doesn't work here due to missing lifetime
// type Instruction = (&str, i32);

fn read_instructions(inp: &str) -> Vec<(&str, i32)> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line_to_instruction(line))
        .collect()
}

fn line_to_instruction(line: &str) -> (&str, i32) {
    let pair: Vec<_> = line.split(" ").collect();
    let arg = pair[1].parse::<i32>().unwrap();

    (pair[0], arg)
}

fn exec(instruction: &(&str, i32), pc: &mut usize, acc: &mut i32) {
    let op = instruction.0;
    let arg = instruction.1;
    match instruction.0 {
        "nop" => {
            *pc += 1;
        }
        "acc" => {
            *pc += 1;
            *acc += arg
        }
        "jmp" => {
            let new_pc = (*pc as i32) + arg;
            if new_pc < 0 {
                panic!("jumping into negative space!");
            }
            *pc = new_pc as usize;
        }
        _ => panic!("invalid instruction {}", op),
    }
}
