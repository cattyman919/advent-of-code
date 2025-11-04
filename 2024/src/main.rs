use std::path::Path;

mod day_1_historian;
mod day_2_red_noses_reports;
mod utils;

fn main() {
    let path = Path::new("input")
        .join("day_2_red_nosed_reports")
        .join("input-1.txt");
    let res = day_2_red_noses_reports::run(path);
    println!("{}", res);
}
