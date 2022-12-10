use ::std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("File read failed!");
    let mut sum = 0;
    let rounds = input.trim().lines();

    for round in rounds {
        let mut hands = round.split(" ");
        let (them, me) = (hands.next().unwrap(), hands.next().unwrap());
        let scores_from_hand = me.as_bytes()[0] as u32 - 'W' as u32;
        let points_from_result = match them {
            "A" if me == "Y" => 6,
            "B" if me == "Z" => 6,
            "C" if me == "X" => 6,

            "A" if me == "X" => 3,
            "B" if me == "Y" => 3,
            "C" if me == "Z" => 3,

            _ => 0,
        };

        sum = sum + scores_from_hand + points_from_result;
    }

    println!("Total: {sum}");
    assert_eq!(sum, 9177);
}
