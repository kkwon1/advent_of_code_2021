// assume that each line will have 12 bits. That is the case in this AoC challenge.

/*
    We want to figure out the most frequent and least frequent value for each position.
    Since they are 0 or 1, if 0 is most freq then 1 must be least freq (vice versa).

    Let's add to array position whenever we see a 1, and subtract whenever we see a 0.
    If positive at end, that's 1. If negative, that's 0. If it's 0, that shouldn't be possible
*/

pub fn part_one(puzzle_input: &[String]) -> u32 {
    let gamma_bits = gamma_rate(puzzle_input);
    let epsilon_bits = epsilon_rate(&gamma_bits);

    let gamma: u32 = u32::from_str_radix(gamma_bits.as_str(), 2).expect("Not a binary number!");
    let epsilon: u32 = u32::from_str_radix(epsilon_bits.as_str(), 2).expect("Not a binary number!");

    gamma * epsilon
}

fn gamma_rate(puzzle_input: &[String]) -> String {
    let mut bit_freq: [i32; 12] = [0; 12];
    for line in puzzle_input {
        let mut index = 0;
        for c in line.chars() {
            match c {
                '0' => {
                    bit_freq[index] = bit_freq[index] - 1;
                    index += 1;
                }
                '1' => {
                    bit_freq[index] = bit_freq[index] + 1;
                    index += 1;
                }
                _ => (),
            }
        }
    }

    let mut gamma_bits = String::new();
    for freq in bit_freq.iter() {
        if *freq > 0 {
            gamma_bits.push('1');
        } else if *freq < 0 {
            gamma_bits.push('0');
        } else {
            panic!("This should never happen!");
        }
    }

    gamma_bits
}

// just invert gamma bits
fn epsilon_rate(gamma_bits: &str) -> String {
    let mut epsilon_bits = String::new();
    for bit in gamma_bits.chars() {
        match bit {
            '0' => epsilon_bits.push('1'),
            '1' => epsilon_bits.push('0'),
            _ => (),
        }
    }
    epsilon_bits
}
