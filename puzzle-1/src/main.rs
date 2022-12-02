use ::std::fs;

fn main() {
    let list = fs::read_to_string("src/input.txt").expect("File read failed!");
    let mut max = list
        .split("\n\n")
        .map(|e| e.lines())
        .map(|e| e.fold(0, |sum, c| sum + c.trim().parse::<i32>().unwrap()))
        .collect::<Vec<i32>>();

    max.sort_by(|a, b| b.cmp(a));

    let top3 = &max[0..3].iter().fold(0, |sum, i| i + sum);

    println!("{top3}");
}
