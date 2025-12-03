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
            $fn(&$inputs
                .get($day - 1)
                .expect(&format!("Failed to get input for day {}", $day)));
        });
        println!("-----");
    }};
}

#[macro_export]
macro_rules! get_inputs {
    () => {{
        let mut __files = std::fs::read_dir("inputs")
            .expect("Failed to read inputs directory")
            .map(|entry| {
                entry
                    .expect("Failed to get entries in input directory")
                    .path()
            })
            .collect::<Vec<_>>();
        __files.sort();
        __files
            .iter()
            .map(|path| {
                std::fs::read_to_string(path).expect("Failed to read entries in input directory")
            })
            .collect::<Vec<String>>()
    }};
}
