use std::time::SystemTime;

pub mod prime_utils;
pub mod digit_utils;

pub fn execute_problem(label: &str, func: fn() -> String) {
    println!("# {}", label);

    let start_time = SystemTime::now();
    let result = func();
    let execution_time = start_time.elapsed().unwrap();

    println!("Result: {}\nDuration: {:?}\n", result, execution_time);
}