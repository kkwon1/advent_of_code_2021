mod day1;
mod day2;
mod helpers;

fn main() {
    // day1();
    day2();
}

fn day1() {
    let count = day1::day1::get_counts(
        "C:\\Users\\kevin\\DevWork\\rust\\advent_of_code\\src\\day1\\day1.txt",
    );
    println!("{}", count);
}

fn day2() {
    let lines: Vec<String> = helpers::file_reader::lines_from_file(
        "C:\\Users\\kevin\\DevWork\\rust\\advent_of_code\\src\\day2\\day2.txt",
    );
    let final_depth: u32 = day2::day2::get_final_depth(lines);
    println!("{}", final_depth);
}
