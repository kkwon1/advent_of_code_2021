mod day1;
mod day2;
mod day3;
mod helpers;

fn main() {
    day1();
    day2();
    day3();
}

fn day1() {
    let _count = day1::day1::get_counts(
        "C:\\Users\\kevin\\DevWork\\rust\\advent_of_code\\src\\day1\\day1.txt",
    );
}

fn day2() {
    let lines: Vec<String> = helpers::file_reader::lines_from_file(
        "C:\\Users\\kevin\\DevWork\\rust\\advent_of_code\\src\\day2\\day2.txt",
    );
    let final_depth: u32 = day2::day2::get_final_depth(&lines);
    let other_final_depth: u32 = day2::day2::single_iteration_solution(&lines);
    assert_eq!(final_depth, other_final_depth);

    let _depth = day2::day2::part_two(&lines);
}

fn day3() {
    let lines: Vec<String> = helpers::file_reader::lines_from_file(
        "C:\\Users\\kevin\\DevWork\\rust\\advent_of_code\\src\\day3\\day3.txt",
    );
    // let power: u32 = day3::day3::part_one(&lines);
    // println!("{}", power);

    let life_support: u32 = day3::day3::part_two(&lines);
    println!("{}", life_support);
}
