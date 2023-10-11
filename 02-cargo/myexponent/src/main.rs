fn main() {
    println!("Hello, world!");
}

fn pow(base: i64, exponent: usize) -> i64 {
    let mut result: i64 = 1;

    if exponent == 0 {
        return 1;
    }

    for _ in 0..exponent {
        result *= base;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::pow;

    #[test]
    fn minus_two_raised_three_there_is_minus_eight() {
        assert_eq!(pow(-2, 3), -8);
    }
}