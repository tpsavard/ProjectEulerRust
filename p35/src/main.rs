/// Project Euler, problem #35
/// > The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
/// > There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
/// > How many circular primes are there below one million?

fn p35() -> String {
    let mut count = 0;
    for n in utils::prime_utils::generate_primes(1_000_000) {
        if is_circular(n) {
            count += 1;
        }
    }
    return count.to_string();
}

fn is_circular(n: i32) -> bool {
    let rotated_values = utils::digit_utils::get_rotated_values(n);
    for rotated_value in rotated_values {
        if !utils::prime_utils::is_prime(rotated_value) {
            return false;
        }
    }
    return true;
}

// ~

fn main() {
    utils::execute_problem("P35", p35);
}
