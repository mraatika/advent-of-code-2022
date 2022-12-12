use ::std::fs;

fn puzzle2_1() {
    let input = fs::read_to_string("src/input.txt").expect("File read failed!");
    let sum = input
        .trim()
        .lines()
        .map(|rounds| rounds.split(" "))
        .map(|mut hands| (hands.next().unwrap(), hands.next().unwrap()))
        .fold(0, |sum, (them, me)| {
            let points_from_hand = me.as_bytes()[0] as u32 - 'W' as u32;
            let points_from_result = match them {
                "A" if me == "Y" => 6,
                "B" if me == "Z" => 6,
                "C" if me == "X" => 6,

                "A" if me == "X" => 3,
                "B" if me == "Y" => 3,
                "C" if me == "Z" => 3,

                _ => 0,
            };

            sum + points_from_hand + points_from_result
        });

    println!("Total: {sum}");
}

fn puzzle2_2() {
    let input = fs::read_to_string("src/input.txt").expect("File read failed!");
    let sum = input
        .trim()
        .lines()
        .map(|rounds| rounds.split(" "))
        .map(|mut hands| (hands.next().unwrap(), hands.next().unwrap()))
        .fold(0, |sum, (them, result)| {
            let points_from_result = match result {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            };

            let my_hand = match them {
                "A" if result == "X" => "C",
                "B" if result == "X" => "A",
                "C" if result == "X" => "B",

                "A" if result == "Z" => "B",
                "B" if result == "Z" => "C",
                "C" if result == "Z" => "A",

                _ => them,
            };

            let points_from_hand = my_hand.as_bytes()[0] as u32 - '@' as u32;

            sum + points_from_hand + points_from_result
        });

    println!("Total: {sum}");
}

fn main() {
    puzzle2_1();
    puzzle2_2();
}
