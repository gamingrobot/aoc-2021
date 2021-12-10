use std::{error::Error};

#[aoc_generator(day3)]
fn gen(input: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    // let mut output = vec![Vec::<u8>::new(); 12];
    // for line in input.lines() {
    //     for (i, c) in line.char_indices() {
    //         output[i].push(c.to_digit(2).unwrap() as u8);
    //     }
    // }
    // Ok(output)
    let mut output = Vec::<Vec::<u8>>::new();
    for line in input.lines() {
        output.push(line.chars().map(|x| x.to_digit(2).unwrap() as u8).collect());
    }
    Ok(output)
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<u8>>) -> u32 {
    //println!("{:?}", input);
    let columns = convert_to_columns(input);
    let mut gamma_bits: Vec<u8> = Vec::new();
    for column in columns {
        let (ones, zeros) = get_counts(&column);
        gamma_bits.push(if ones > zeros { 1 } else { 0 });
    }
    let gamma = convert_bits_to_bytes(&gamma_bits);
    let mut epsilon = !gamma;
    epsilon ^= 0b1111_0000_0000_0000; //set the top 4 bits to 0
    println!("gamma: {} {:0>8b}", gamma, gamma);
    println!("epsilon: {} {:0>8b}", epsilon, epsilon);
    gamma as u32 * epsilon as u32
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<u8>>) -> u32 {
    let mut current_bit = 0;
    let mut current = input.clone(); //gross
    while current_bit < 12 {
        let columns = convert_to_columns(&current); //also gross
        let (ones, zeros) = get_counts(&columns[current_bit]);
        println!("Ones: {:?} Zeros: {:?} ", ones, zeros);
        let common_bit = if ones < zeros { 0 } else { 1 };
        current = current.into_iter().filter(|x| x[current_bit] == common_bit).collect();
        if current.len() == 1 {
            break;
        }
        println!("Common Bit: {:?}", common_bit);
        println!("Current Size: {:?}", current.len());
        current_bit += 1;
    }
    println!("Oxygen: {:?}", current);
    let oxygen = convert_bits_to_bytes(&current[0]);
    println!("oxygen: {} {:0>8b}", oxygen, oxygen);
    current = input.clone(); //gross
    current_bit = 0;
    while current_bit < 12 {
        let columns = convert_to_columns(&current); //also gross
        //println!("Current column {:?}", columns[current_bit]);
        let (ones, zeros) = get_counts(&columns[current_bit]);
        println!("Ones: {:?} Zeros: {:?} ", ones, zeros);
        let uncommon_bit = if ones < zeros { 1 } else { 0 };
        println!("Uncommon Bit: {:?}", uncommon_bit);
        current = current.into_iter().filter(|x| x[current_bit] == uncommon_bit).collect();
        //println!("Current: {:?}", current);
        if current.len() == 1 {
            break;
        }
        println!("Current Size: {:?}", current.len());
        current_bit += 1;
    }
    println!("Co2: {:?}", current);
    let co2 = convert_bits_to_bytes(&current[0]);
    println!("Co2: {} {:0>8b}", co2, co2);
    oxygen as u32 * co2 as u32
}

pub fn convert_to_columns(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut columns = vec![Vec::<u8>::new(); 12];
    for line in input {
        for (i, c) in line.iter().enumerate() {
            columns[i].push(*c);
        }
    }
    columns
}

pub fn convert_bits_to_bytes(input: &[u8]) -> u16 {
    input.iter().fold(0, |result: u16, &bit| {
        (result << 1) ^ bit as u16
    })
}

pub fn get_counts(input: &[u8]) -> (u32, u32)
{
    input.iter().fold((0, 0), |(ones, zeros), &x| {
        if x == 1 {
            (ones + 1, zeros)
        } else {
            (ones, zeros + 1)
        }
    })
}