// This solution iterates over entire input twice. I can do it in one go but I just wanted to
// play around with enums. I will implement a separate solution to do it in 1 go.

pub enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
    Invalid,
}

pub fn get_final_depth(puzzle_input: &[String]) -> u32 {
    let directions: Vec<Direction> = convert_string_to_direction(puzzle_input);

    let mut h_pos = 0;
    let mut depth = 0;
    for direction in directions {
        match direction {
            Direction::Forward(val) => {
                h_pos += val;
            }
            Direction::Up(val) => {
                depth -= val;
            }
            Direction::Down(val) => {
                depth += val;
            }
            _ => (),
        }
    }

    h_pos * depth
}

fn convert_string_to_direction(puzzle_input: &[String]) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    for line in puzzle_input {
        let input_split: Vec<&str> = line.split(" ").collect();
        let value: u32 = input_split[1].parse::<u32>().unwrap();

        match input_split[0] {
            "forward" => {
                directions.push(Direction::Forward(value));
            }
            "up" => {
                directions.push(Direction::Up(value));
            }
            "down" => {
                directions.push(Direction::Down(value));
            }
            _ => directions.push(Direction::Invalid),
        }
    }

    directions
}

pub fn single_iteration_solution(puzzle_input: &[String]) -> u32 {
    let mut h_pos = 0;
    let mut depth = 0;

    for line in puzzle_input {
        let input_split: Vec<&str> = line.split(" ").collect();
        let val: u32 = input_split[1].parse::<u32>().unwrap();

        match input_split[0] {
            "forward" => {
                h_pos += val;
            }
            "up" => {
                depth -= val;
            }
            "down" => {
                depth += val;
            }
            _ => (),
        }
    }
    h_pos * depth
}
