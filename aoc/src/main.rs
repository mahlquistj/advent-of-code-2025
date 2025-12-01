mod macros;

fn main() {
    println!("Advent of Code 2025 solutions");
    let inputs = time!("Get inputs", { get_inputs!() });
    println!("----");
    time!("All", {
        day!(1, day1, inputs);
        day!(2, day2, inputs);
        day!(3, day3, inputs);
        day!(4, day4, inputs);
        day!(5, day5, inputs);
        day!(6, day6, inputs);
        day!(7, day7, inputs);
        day!(8, day8, inputs);
        day!(9, day9, inputs);
        day!(10, day10, inputs);
        day!(11, day11, inputs);
        day!(12, day12, inputs);
    });
}

fn day1(input: &str) {
    let instructions = day1::DialInstructions::parse(input).unwrap();

    // Solution 1
    let mut counter = 0;
    time!("day1#1", {
        let mut lock = day1::DialLock::new(50, 99);
        instructions.apply_to_lock_with_fn(&mut lock, |current, _| {
            if current == 0 {
                counter += 1;
            }
        });
    });
    println!("Solution 1: {}", counter);

    let mut counter = 0;
    time!("day1#2", {
        let mut lock = day1::DialLock::new(50, 99);
        instructions.apply_to_lock_with_fn(&mut lock, |current, resets| {
            counter += resets;
            if current == 0 {
                counter += 1;
            }
        });
    });
    println!("Solution 2: {}", counter);
}

fn day2(_input: &str) {
    println!("Not done yet");
}

fn day3(_input: &str) {
    println!("Not done yet");
}

fn day4(_input: &str) {
    println!("Not done yet");
}

fn day5(_input: &str) {
    println!("Not done yet");
}

fn day6(_input: &str) {
    println!("Not done yet");
}

fn day7(_input: &str) {
    println!("Not done yet");
}

fn day8(_input: &str) {
    println!("Not done yet");
}

fn day9(_input: &str) {
    println!("Not done yet");
}

fn day10(_input: &str) {
    println!("Not done yet");
}

fn day11(_input: &str) {
    println!("Not done yet");
}

fn day12(_input: &str) {
    println!("Not done yet");
}
