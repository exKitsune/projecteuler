// greatest common denominator
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b
    }
    if b == 0 {
        return a
    }

    if a < b { let c = a; a = b; b = c; }
    #[cfg(test)]
    println!("{}, {}", a, b);
    gcd(b, a % b)
}

// least common multiple
pub fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils_gcd() {
        assert!(gcd(10, 12) == 2);
        assert!(gcd(14535, 1535) == 5);
        assert!(gcd(17, 2323) == 1);
    }

    #[test]
    fn utils_lcd() {
        assert!(lcm(12, 15) == 60);
        assert!(lcm(3, 9) == 9);
        assert!(lcm(135135, 3892) == 75135060);
    }
}