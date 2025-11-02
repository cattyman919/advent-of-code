use std::path::Path;

mod day_1_historian;
mod utils;

fn main() {
    let path = Path::new("input")
        .join("day_1_historian")
        .join("input-2.txt");
    let res = day_1_historian::run_part2(path);
    println!("{}", res);
}
