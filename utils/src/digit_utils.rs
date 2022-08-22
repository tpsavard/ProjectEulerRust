pub fn get_rotated_values(n: i32) -> Vec<i32> {
    let mut rotated_values: Vec<i32> = vec![n];
    let mut rotated_digits = ntod(n);
    loop {
        let first_digit = rotated_digits.remove(0);
        rotated_digits.push(first_digit);
        
        let rotated_value = dton(&rotated_digits);
        if rotated_value == n {
            break;
        } else {
            rotated_values.push(rotated_value);
        }
    } 
    return rotated_values;
}

pub fn ntod(mut n: i32) -> Vec<i32> {
    if n == 0 {
        return vec![0];
    }
    let mut digits: Vec<i32> = Vec::new();
    while n > 0 {
        digits.insert(0, n % 10);
        n /= 10;
    }
    return digits;
}

pub fn dton(digits: &[i32]) -> i32 {
    let mut n = 0;
    for digit in digits.iter() {
        n *= 10;
        n += digit;
    }
    return n;
}