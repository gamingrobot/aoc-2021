use std::error::Error;

#[aoc_generator(day3)]
fn gen(input: &str) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let mut output = vec![Vec::new(); 12];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            output[i].push(c.to_digit(2).unwrap());
        }
    }
    Ok(output)
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    //println!("{:?}", input);
    let mut gamma_bits: Vec<u16> = Vec::new();
    for column in input {
        let (ones, zeros) = column.iter().fold((0, 0), |(ones, zeros), &x| {
            if x == 1 {
                (ones + 1, zeros)
            } else {
                (ones, zeros + 1)
            }
        });
        if ones > zeros{
            gamma_bits.push(1);
        }
        else {
            gamma_bits.push(0);
        }
    }
    let gamma = gamma_bits.iter()
        .fold(0, |result: u16, &bit| {
            (result << 1) ^ bit
        });
    let mut epsilon = !gamma;
    epsilon ^= 0b1111_0000_0000_0000; //set the top 4 bits to 0
    println!("gamma: {} {:0>8b}", gamma, gamma);
    println!("epsilon: {} {:0>8b}", epsilon, epsilon);
    gamma as u32 * epsilon as u32
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    1
}