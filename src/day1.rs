use std::num::ParseIntError;

#[aoc_generator(day1)]
fn gen(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1, first)]
pub fn part1(input: &[u32]) -> u32 {
    let mut prev: u32 = *input.first().unwrap();
    let mut count = 0;
    for i in input {
        if i > &prev {
            count += 1;
        }
        prev = *i;
    }
    count
}

#[aoc(day1, part1, iterator)]
pub fn part1_iterator(input: &[u32]) -> usize {
    input.windows(2).filter(|&w| w[1] > w[0]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> usize {
    input
        .windows(3)
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|&w| (w[1][0] + w[1][1] + w[1][2]) > (w[0][0] + w[0][1] + w[0][2]))
        .count()
}
