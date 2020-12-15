
use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};

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
            "N" => { Day12::North(units) },
            "S" => { Day12::South(units) },
            "E" => { Day12::East(units) },
            "W" => { Day12::West(units) },
            "L" => { Day12::Left(units) },
            "R" => { Day12::Right(units) },
            "F" => { Day12::Forward(units) }
            _ => unreachable!()
        });
    }
    ret
}

fn cmd_part1(cmd: &Day12, coords: &mut (i32, i32), facing: &mut (i32, i32)) {
    match *cmd {
        Day12::North(u) => {
            coords.1 -= u;
        },
        Day12::South(u) => {
            coords.1 += u;
        },
        Day12::East(u) => {
            coords.0 += u;
        },
        Day12::West(u) => {
            coords.0 -= u;
        },
        Day12::Left(u) => {
            let u = 4 - u / 90;
            for _ in 0..u {
                *facing = (-facing.1, facing.0);
            }
        },
        Day12::Right(u) => {
            let u = u / 90;
            for _ in 0..u {
                *facing = (-facing.1, facing.0);
            }
        },
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
        },
        Day12::South(u) => {
            waypoint.1 += u;
        },
        Day12::East(u) => {
            waypoint.0 += u;
        },
        Day12::West(u) => {
            waypoint.0 -= u;
        },
        Day12::Left(u) => {
            let u = 4 - u / 90;
            for _ in 0..u {
                *waypoint = (-waypoint.1, waypoint.0);
            }
        },
        Day12::Right(u) => {
            let u = u / 90;
            for _ in 0..u {
                *waypoint = (-waypoint.1, waypoint.0);
            }
        },
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

aoc_lib!{ year = 2020 }