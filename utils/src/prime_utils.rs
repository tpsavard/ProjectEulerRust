/// Uses 6k+/-1 optimization
/// From https://en.wikipedia.org/wiki/Primality_test
pub fn is_prime(n: i32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

/// Based on Sieve of Sundaram
/// From https://www.baeldung.com/cs/prime-number-algorithms#sieve-of-sundaram
pub fn generate_primes(max: i32) -> Vec<i32> {
    if max < 2 {
        return vec![];
    }

    let k = (((max - 1) / 2) as f64).floor();

    let mut range = vec![true; (k + 1.0) as usize];
    range[0] = false;
    for i in 1..(k.sqrt() as i32) {
        let mut j = i;
        while (i + j + (2 * i * j)) <= (k as i32) {
            range[(i + j + (2 * i * j)) as usize] = false;
            j += 1;
        }
    }

    let mut primes: Vec<i32> = Vec::new();
    for i in 0..range.len() {
        if range[i] {
            primes.push(i as i32);
        }
    }
    for _ in 0..primes.len() {
        primes.push((primes[0] * 2) + 1);
        primes.remove(0);
    }
    primes.insert(0, 2);

    return primes;
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn is_prime_test() {
        // TODO
    }

    #[test]
    fn generate_primes_test() {
        // TODO
    }
}
