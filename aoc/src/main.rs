use std::str::FromStr;

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
    let instructions = time!("day1#parse", {
        day1::DialInstructions::parse(input).unwrap()
    });

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

fn day2(input: &str) {
    let checker = time!("day2#parse", { day2::IdChecker::from_ids(input).unwrap() });

    let angel_numbers_sum = time!("day2#1", { checker.sum_angel_numbers() });
    println!("Solution 1: {angel_numbers_sum}");

    let invalid_ids_sum = time!("day2#2", { checker.sum_invalid_ids() });
    println!("Solution 2: {invalid_ids_sum}");
}

fn day3(input: &str) {
    let emergency_power = time!("day3#parse", {
        day3::EmergencyPower::from_str(input).unwrap()
    });

    let max_joltage = time!("day3#1", { emergency_power.max_joltage() });
    println!("Solution 1: {max_joltage}");

    let max_joltage_unsafe = time!("day3#2", { emergency_power.max_joltage_unsafe::<12>() });
    println!("Solution 2: {max_joltage_unsafe}");
}

fn day4(input: &str) {
    let mut storage_room = time!("day4#parse", {
        day4::StorageRoom::from_str(input).unwrap()
    });

    let accessible_rolls = time!("day4#1", { storage_room.count_accessible_paper_rolls() });
    println!("Solution 1: {accessible_rolls}");

    let all_accessible_rolls = time!("day4#2", {
        storage_room.count_accessible_paper_rolls_incrementally()
    });
    println!("Solution 2: {all_accessible_rolls}");
}

fn day5(input: &str) {
    let database = time!("day5#parse", { day5::Database::from_str(input).unwrap() });

    let fresh_ingredients = time!("day5#1", { database.count_fresh_ingredients() });
    println!("Solution 1: {fresh_ingredients}");

    let fresh_ingredient_ids = time!("day5#2", { database.count_fresh_ids() });
    println!("Solution 2: {fresh_ingredient_ids}");
}

fn day6(input: &str) {
    let worksheet = time!("day6#parse", { day6::Worksheet::from_str(input).unwrap() });

    let horizontal_sum = time!("day6#1", { worksheet.solve_horizontal_problems_and_sum() });
    println!("Solution 1: {horizontal_sum}");

    let vertical_sum = time!("day6#1", { worksheet.solve_vertical_problems_and_sum() });
    println!("Solution 2: {vertical_sum}");
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
