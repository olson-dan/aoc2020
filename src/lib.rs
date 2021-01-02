use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
use std::collections::HashMap;

enum Day12 {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[aoc_generator(day12)]
fn day12_input(s: &str) -> Vec<Day12> {
    let mut ret = Vec::new();
    for line in s.lines() {
        let (cmd, units) = line.split_at(1);
        let units = units.parse::<i32>().unwrap();
        ret.push(match cmd {
            "N" => Day12::North(units),
            "S" => Day12::South(units),
            "E" => Day12::East(units),
            "W" => Day12::West(units),
            "L" => Day12::Left(units),
            "R" => Day12::Right(units),
            "F" => Day12::Forward(units),
            _ => unreachable!(),
        });
    }
    ret
}

fn cmd_part1(cmd: &Day12, coords: &mut (i32, i32), facing: &mut (i32, i32)) {
    match *cmd {
        Day12::North(u) => {
            coords.1 -= u;
        }
        Day12::South(u) => {
            coords.1 += u;
        }
        Day12::East(u) => {
            coords.0 += u;
        }
        Day12::West(u) => {
            coords.0 -= u;
        }
        Day12::Left(u) => {
            let u = 4 - u / 90;
            for _ in 0..u {
                *facing = (-facing.1, facing.0);
            }
        }
        Day12::Right(u) => {
            let u = u / 90;
            for _ in 0..u {
                *facing = (-facing.1, facing.0);
            }
        }
        Day12::Forward(u) => {
            coords.0 += facing.0 * u;
            coords.1 += facing.1 * u;
        }
    }
}

#[aoc(day12, part1)]
fn day12_part1(input: &[Day12]) -> i32 {
    let mut coords = (0i32, 0i32);
    let mut facing = (1i32, 0i32);

    for cmd in input {
        cmd_part1(cmd, &mut coords, &mut facing);
    }
    coords.0.abs() + coords.1.abs()
}

fn cmd_part2(cmd: &Day12, coords: &mut (i32, i32), waypoint: &mut (i32, i32)) {
    match *cmd {
        Day12::North(u) => {
            waypoint.1 -= u;
        }
        Day12::South(u) => {
            waypoint.1 += u;
        }
        Day12::East(u) => {
            waypoint.0 += u;
        }
        Day12::West(u) => {
            waypoint.0 -= u;
        }
        Day12::Left(u) => {
            let u = 4 - u / 90;
            for _ in 0..u {
                *waypoint = (-waypoint.1, waypoint.0);
            }
        }
        Day12::Right(u) => {
            let u = u / 90;
            for _ in 0..u {
                *waypoint = (-waypoint.1, waypoint.0);
            }
        }
        Day12::Forward(u) => {
            coords.0 += waypoint.0 * u;
            coords.1 += waypoint.1 * u;
        }
    }
}

#[aoc(day12, part2)]
fn day12_part2(input: &[Day12]) -> i32 {
    let mut coords = (0i32, 0i32);
    let mut waypoint = (10i32, -1i32);

    for cmd in input {
        cmd_part2(cmd, &mut coords, &mut waypoint);
    }
    coords.0.abs() + coords.1.abs()
}

#[test]
fn day12_test() {
    let input = "F10\nN3\nF7\nR90\nF11";
    let input = day12_input(&input);
    assert_eq!(25, day12_part1(&input));
    assert_eq!(286, day12_part2(&input));
}

struct Day13 {
    target: usize,
    busses: Vec<usize>,
}
#[aoc_generator(day13)]
fn day13_input(s: &str) -> Day13 {
    let mut lines = s.lines();
    let target = lines.next().unwrap().parse::<usize>().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| {
            if x == "x" {
                Some(0)
            } else {
                Some(x.parse::<usize>().unwrap())
            }
        })
        .collect::<Vec<_>>();
    Day13 { target, busses }
}

#[aoc(day13, part1)]
fn day13_part1(input: &Day13) -> usize {
    for x in input.target.. {
        for bus in &input.busses {
            if *bus == 0 {
                continue;
            }
            if x % bus == 0 {
                return *bus * (x - input.target);
            }
        }
    }
    unreachable!()
}

#[aoc(day13, part2)]
fn day13_part2(input: &Day13) -> usize {
    let offsets = input
        .busses
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != 0)
        .collect::<Vec<_>>();
    let (offset, step) = *offsets.iter().max_by_key(|(_, x)| *x).unwrap();
    let mut accum = 0;
    loop {
        accum += *step;
        let start = accum - offset;
        let mut valid = true;
        for (i, x) in &offsets {
            if (start + *i) % **x != 0 {
                valid = false;
                break;
            }
        }
        if valid {
            return start;
        }
    }
}

#[test]
fn day13_test() {
    let input = "939\n7,13,x,x,59,x,31,19\n";
    let input = day13_input(input);
    assert_eq!(295, day13_part1(&input));
    assert_eq!(1068781, day13_part2(&input));
    let input = "0\n1789,37,47,1889";
    let input = day13_input(input);
    assert_eq!(1202161486, day13_part2(&input));
}

struct Day14 {
    commands: Vec<(u64, u64, u64, u64)>,
}

#[aoc_generator(day14)]
fn day14_generate(s: &str) -> Day14 {
    let mut zeroes = 0u64;
    let mut ones = 0u64;

    let mut commands = Vec::new();
    for l in s.lines() {
        if l.starts_with("mask") {
            zeroes = 0;
            ones = 0;
            let mask = l.split(" = ").nth(1).unwrap();
            for (i, c) in mask.chars().enumerate() {
                if c == '0' {
                    zeroes |= 1 << (35 - i);
                } else if c == '1' {
                    ones |= 1 << (35 - i);
                }
            }
        } else {
            let mut split = l.trim_end().split(" = ");
            let address = split.next().unwrap();
            let address = address.trim_end_matches(']');
            let address = address.split('[').nth(1).unwrap();
            let address = address.parse::<u64>().unwrap();
            let value = split.next().unwrap().parse::<u64>().unwrap();
            commands.push((address, value, zeroes, ones));
        }
    }
    Day14 { commands }
}

#[aoc(day14, part1)]
fn day14_part1(input: &Day14) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for (address, value, zeroes, ones) in &input.commands {
        let value = (*value | *ones) & !*zeroes;
        let entry = mem.entry(*address).or_default();
        *entry = value;
    }
    mem.iter().map(|(_, v)| *v).sum()
}

#[aoc(day14, part2)]
fn day14_part2(input: &Day14) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for (address, value, zeroes, ones) in &input.commands {
        let floating: u64 = (1 << 36) - 1 & !(zeroes | ones);
        let num_addresses = 1 << floating.count_ones();
        for x in 0u64..num_addresses {
            let mut floating_address = 0;
            let mut floating_bit = 0;
            for i in 0..36 {
                let bit = 1 << i;
                if ones & bit != 0 {
                    floating_address |= bit;
                } else if zeroes & bit != 0 {
                    floating_address |= address & bit;
                } else {
                    assert!(floating & bit != 0);
                    if x & (1 << floating_bit) != 0 {
                        floating_address |= bit;
                    }
                    floating_bit += 1;
                }
            }
            let entry = mem.entry(floating_address).or_default();
            *entry = *value;
        }
    }
    mem.iter().map(|(_, v)| *v).sum()
}

#[test]
fn day14_test() {
    let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1\n";
    let input = day14_generate(input);
    assert_eq!(208, day14_part2(&input));
}
aoc_lib! { year = 2020 }
