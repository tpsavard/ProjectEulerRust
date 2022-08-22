/// Project Euler, problem #36
/// > The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
/// > Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
/// > (Please note that the palindromic number, in either base, may not include leading zeros.)

fn p36() -> String {
    let mut palindromes: Vec<i32> = vec![];
    for n in 1..1_000_000 {
        if is_dec_bin_palindrome(n) {
            palindromes.push(n);
        }
    }
    let sum: i32 = palindromes.iter().sum();
    return sum.to_string();
}

fn is_dec_bin_palindrome(n: i32) -> bool {
    let is_dec_palindrome = is_palindrome(&format!("{n}"));
    let is_bin_palindrome = is_palindrome(&format!("{n:b}"));
    
    // println!("{n} / {}, {n:b} / {}", is_dec_palindrome, is_bin_palindrome);

    return is_dec_palindrome && is_bin_palindrome;
}

fn is_palindrome(s: &str) -> bool {
    if s.chars().count() < 2 {
        return true;
    }

    let i = if s.chars().count() % 2 == 0 { 1 } else { 2 };
    let j = (s.chars().count() - i) / 2;
    for n in 0..=j {
        if s.chars().nth(n) != s.chars().nth_back(n) {
            return false;
        }
    }

    return true;
}

// ~

fn main() {
    utils::execute_problem("P36", p36);
}
