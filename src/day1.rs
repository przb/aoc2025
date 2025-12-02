use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut dial = 50;
    let mut count = 0;
    input
        .lines()
        .map(|l| {
            let mul = match l.as_bytes().first().unwrap() {
                b'L' => -1,
                b'R' => 1,
                _ => panic!("non lr"),
            };
            let rots: i32 = l[1..].parse().unwrap();
            rots * mul
        })
        .for_each(|rot| {
            dial += rot;
            while dial < 0 {
                dial += 100
            }
            while dial >= 100 {
                dial = dial % 100;
            }
            if dial == 0 {
                count += 1;
            }
        });
    count
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut dial = 50;
    let mut count = 0;
    input
        .lines()
        .map(|l| {
            let mul = match l.as_bytes().first().unwrap() {
                b'L' => -1,
                b'R' => 1,
                _ => panic!("non lr"),
            };
            let rots: i32 = l[1..].parse().unwrap();
            rots * mul
        })
        .for_each(|mut rot| {
            // dial += rot;
            // while dial > 100 {
            //     dial -= 100;
            //     count += 1;
            // }
            // while dial < 0 {
            //     dial += 100;
            //     count += 1;
            // }
            // if dial == 0 {
            //     count += 1
            // }
            // if dial == 100 {
            //     dial = 0;
            //     count += 1;
            // }

            // this is terrible
            while rot > 0 {
                dial += 1;
                rot -= 1;
                if dial == 100 {
                    dial = 0;
                }
                if dial == -1 {
                    dial = 99;
                }
                if dial == 0 {
                    count += 1;
                }
            }
            while rot < 0 {
                dial -= 1;
                rot += 1;
                if dial == 100 {
                    dial = 0;
                }
                if dial == -1 {
                    dial = 99;
                }
                if dial == 0 {
                    count += 1;
                }
            }
        });
    count
}

#[cfg(test)]
mod tests {

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    #[test]
    fn example_d1p1() {
        let ans = super::part1(INPUT);
        assert_eq!(ans, 3)
    }
    #[test]
    fn example_d1p2() {
        let ans = super::part2(INPUT);
        assert_eq!(ans, 6)
    }

    const REAL_INPUT: &str = include_str!("../input/2025/day1.txt");

    #[test]
    fn real_d1p1() {
        let ans = super::part1(REAL_INPUT);
        assert_eq!(ans, 1026)
    }
    #[test]
    fn real_d1p2() {
        let ans = super::part2(REAL_INPUT);
        assert_eq!(ans, 5923)
    }
}
