use std::collections::HashMap;

/// # [Project Euler Problem #39: Integer Right Triangles](https://projecteuler.net/problem=39)

const MIN_P: i32 = 12; // 3^2 + 4^2 = 5^2
const MAX_P: i32 = 1000;

fn p39() -> String {
    let mut valid_triangles: HashMap<i32, i32> = HashMap::new();

    for p in MIN_P..=MAX_P {
        for a in 1..p {
            for b in 1..(p - a) {
                let c: i32 = p - a - b;
                if (c * c) == (a * a) + (b * b) {
                    match valid_triangles.get(&p) {
                        Some(v) => valid_triangles.insert(p, v + 1),
                        None => valid_triangles.insert(p, 1),
                    };
                }
            }
        }
    }

    let mut max_k = i32::min_value();
    let mut max_v = i32::min_value();
    for (k, v) in valid_triangles.into_iter() {
        if v > max_v {
            max_k = k;
            max_v = v;
        }
    }

    return format!("(P: {}, count: {})", max_k, max_v);
}

// ~

fn main() {
    utils::execute_problem("P39", p39);
}
