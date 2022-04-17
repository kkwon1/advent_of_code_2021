// assume that each line will have 12 bits. That is the case in this AoC challenge.

/*
    We want to figure out the most frequent and least frequent value for each position.
    Since they are 0 or 1, if 0 is most freq then 1 must be least freq (vice versa).

    Let's add to array position whenever we see a 1, and subtract whenever we see a 0.
    If positive at end, that's 1. If negative, that's 0. If it's 0, that shouldn't be possible
*/

pub fn part_one(puzzle_input: &[String]) -> u32 {
    let gamma_bits = gamma_rate(puzzle_input);
    let epsilon_bits = invert_bits(&gamma_bits);

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
fn invert_bits(gamma_bits: &str) -> String {
    let mut inverted_bits = String::new();
    for bit in gamma_bits.chars() {
        match bit {
            '0' => inverted_bits.push('1'),
            '1' => inverted_bits.push('0'),
            _ => (),
        }
    }
    inverted_bits
}

// =================================================
//                PART TWO BELOW
// =================================================

// knowing we have 12 bits in each number and 1000 numbers, let's create a 2D array with known sizes.
// to optimize, we should just build the 2D array while reading from the file directly instead of converting to 2D array after.

const NUM_BITS: usize = 12;
const NUM_ROWS: usize = 1000;

pub fn part_two(puzzle_input: &[String]) -> u32 {
    let mut puzzle_array = puzzle_to_array(puzzle_input);
    let oxygen_rating_str = get_oxygen_rating(puzzle_array);

    let mut puzzle_array = puzzle_to_array(puzzle_input);
    let co2_scrubber_rating_str = get_co2_scrubber_rating(puzzle_array);

    let oxygen_rating: u32 =
        u32::from_str_radix(oxygen_rating_str.as_str(), 2).expect("Not a binary number!");
    let co2_scrubber_rating: u32 =
        u32::from_str_radix(co2_scrubber_rating_str.as_str(), 2).expect("Not a binary number!");

    println!("{}", oxygen_rating);
    println!("{}", co2_scrubber_rating);

    oxygen_rating * co2_scrubber_rating
}

fn puzzle_to_array(puzzle_input: &[String]) -> Vec<Vec<char>> {
    let mut array = vec![vec!['0'; NUM_BITS]; NUM_ROWS];
    for (i, line) in puzzle_input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            array[i][j] = c;
        }
    }

    array
}

fn get_oxygen_rating(mut puzzle_array: Vec<Vec<char>>) -> String {
    // We know each number is 12 bits long

    let mut oxygen_rating: [char; NUM_BITS] = ['0'; NUM_BITS];

    for i in 0..NUM_BITS {
        let (zeros, ones): (Vec<Vec<char>>, Vec<Vec<char>>) =
            puzzle_array.into_iter().partition(|row| row[i] == '0');
        if zeros.len() > ones.len() {
            oxygen_rating[i] = '0';
            puzzle_array = zeros;
        } else if zeros.len() < ones.len() {
            oxygen_rating[i] = '1';
            puzzle_array = ones;
        } else {
            oxygen_rating[i] = '1';
            puzzle_array = ones;
        }
    }

    println!("oxygen: {:?} \n", oxygen_rating);
    oxygen_rating.into_iter().collect()
}

fn get_co2_scrubber_rating(mut puzzle_array: Vec<Vec<char>>) -> String {
    // We know each number is 12 bits long
    let mut co2_scrubber_rating: [char; NUM_BITS] = ['0'; NUM_BITS];

    let mut stopped: bool = false;

    for i in 0..NUM_BITS {
        let (zeros, ones): (Vec<Vec<char>>, Vec<Vec<char>>) =
            puzzle_array.into_iter().partition(|row| row[i] == '0');

        if zeros.len() > ones.len() {
            co2_scrubber_rating[i] = '1';
            puzzle_array = ones;
        } else if zeros.len() < ones.len() {
            co2_scrubber_rating[i] = '0';
            puzzle_array = zeros;
        } else {
            co2_scrubber_rating[i] = '0';
            puzzle_array = zeros;
        }

        if puzzle_array.len() == 1 {
            stopped = true;
            break;
        }
    }

    if stopped {
        for (i, character) in puzzle_array[0].iter().enumerate() {
            co2_scrubber_rating[i] = *character;
        }
    }

    println!("co2: {:?} \n", co2_scrubber_rating);
    co2_scrubber_rating.into_iter().collect()
}

// initial answer is 2365704 too low
// second answer is 4255548 too high
