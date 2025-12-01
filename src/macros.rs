#[macro_export]
macro_rules! time {
    ($name:expr, $block:block) => {{
        let __start = std::time::Instant::now();
        let __result = { $block };
        println!("[TIMING] '{}' took: {:?}", $name, __start.elapsed());
        __result
    }};

    ($name:expr, $fn:ident) => {
        time!($name, { $fn() })
    };
}

#[macro_export]
macro_rules! day {
    ($day:tt, $fn:ident, $inputs:ident) => {{
        println!("# Day {}", $day);
        time!(format!("Day {}", $day), {
            $fn(&$inputs[$day - 1]);
        });
        println!("-----");
    }};
}

#[macro_export]
macro_rules! get_inputs {
    () => {
        std::fs::read_dir("inputs")
            .expect("Failed to read inputs directory")
            .map(|entry| {
                std::fs::read_to_string(
                    entry
                        .expect("Failed to get entries in input directory")
                        .path(),
                )
                .expect("Failed to read entries in input directory")
            })
            .collect::<Vec<String>>()
    };
}
