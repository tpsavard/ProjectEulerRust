use utils::digit_utils::{dton, ntod};

/// # [Project Euler Problem #38: Pandigital Multiples](https://projecteuler.net/problem=38)
/// ## Notes
/// *Naive Efficiencies:*
/// - Halt candidate check on 0 digit
/// - Halt candidate check on >9 digit product
/// *Key Efficiency:* Upper limit of candidate values
/// - *Naive upper limit:* max pandigital number, 987654321
/// - *For candidates that must be multiplied by [1,2]:* must be <5 digits, as 2 5-digit appended numbers would have 10 digits total

const INPUT_UPPER_LIMIT: i32 = 9999;

fn p38() -> String {
    let mut lpd_index: i32 = 0;
    let mut lpd: i32 = 0;
    for i in 0..=INPUT_UPPER_LIMIT {
        if let Some(cand) = get_pandigital_val(i) {
            if cand > lpd {
                lpd = cand;
                lpd_index = i;
            }
        }
    }
    return format!("{}, {}", lpd, lpd_index);
}

fn get_pandigital_val(n: i32) -> Option<i32> {
    let mut digits = Vec::new();
    for i in 1.. {
        digits.append(&mut ntod(n * i));
        if digits.len() == 9 { break; }
        if digits.len() > 9 { return None; }
    }
    if (is_pandigital(&digits, &mut[false; 9])) {
        return Some(dton(&digits));
    } else {
        return None;
    }
}

fn is_pandigital(digits: &[i32], seen_vals: &mut[bool]) -> bool {
    if let Some((head, tail)) = digits.split_first() {
        if *head == 0 { return false; }
        let i = (*head as usize) - 1;
        if !seen_vals[i] {
            seen_vals[i] = true;
            return is_pandigital(tail, seen_vals);
        } else {
            return false;
        }
    } else {
        return seen_vals.iter().fold(true, |acc, b| acc && *b);
    }
}

fn main() {
    utils::execute_problem("P38", p38);
}
