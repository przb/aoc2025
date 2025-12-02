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
            println!("dial: {dial}");
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

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let ans = super::part1(input);
        assert_eq!(ans, 3)
    }
}
