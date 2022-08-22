use utils::prime_utils::generate_primes;
use utils::prime_utils::is_prime;
use utils::digit_utils::ntod;
use utils::digit_utils::dton;

/// Project Euler, problem #37
/// > The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, 
/// > and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
/// > Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
/// > NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

fn p37() -> String {
    let mut t_primes: Vec<i32> = vec![];
    for n in generate_primes(1_000_000) {
        if t_primes.len() >= 11 {
            break;
        }
        let nd = ntod(n);
        if nd.len() < 2 {
            continue;
        }
        if is_truncatable_prime(&nd) {
            t_primes.push(n);
        }
    }

    let t_prime_sum: i32 = (&t_primes).iter().sum();
    return format!("{:?}, {}", t_primes, t_prime_sum);
}

fn is_truncatable_prime(nd: &[i32]) -> bool {
    // Truncate from the left: [1, 2, 3] -> [2, 3] -> [3]
    for l in 0..nd.len() {
        let s = dton(&nd[l..]);

        if !is_prime(s) {
            return false;
        }
    }

    // Truncate from the left: [1, 2, 3] -> [1, 2] -> [1]
    for l in (0..nd.len()).rev() {
        let s = dton(&nd[..(l + 1)]);

        if !is_prime(s) {
            return false;
        }
    }

    return true;
}

// ~

fn main() {
    utils::execute_problem("P37", p37);
}