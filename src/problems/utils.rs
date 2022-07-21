// https://www.baeldung.com/cs/prime-number-algorithms
// generate primes smaller than n
#[allow(non_snake_case)]
pub fn atkin_sieve(n: u64) -> Vec<u64> {
    let mut number_array: Vec<bool> = vec![false; n as usize];

    let M1 = vec![1, 13, 17, 29, 37, 41, 49, 53];
    for x in 1..=(n as f64).sqrt().floor() as u64 {
        for y in (1..=(n as f64).sqrt().floor() as u64).step_by(2) {
            let m = 4 * x.pow(2) + y.pow(2);
            if M1.contains(&(m % 60)) && m <= n {
                number_array[m as usize] = !number_array[m as usize];
            }
        }
    }

    let M2 = vec![7, 19, 31, 43];
    for x in (1..=(n as f64).sqrt().floor() as u64).step_by(2) {
        for y in (2..=(n as f64).sqrt().floor() as u64).step_by(2) { 
            let m = 3 * x.pow(2) + y.pow(2);
            if M2.contains(&(m % 60)) && m <= n {
                number_array[m as usize] = !number_array[m as usize];
            }
        }
    }
    
    let M3 = vec![11, 23, 47, 59];
    for x in 2..=(n as f64).sqrt().floor() as u64 {
        for y in (1..=x-1).rev().step_by(2) {
            let m = 3 * x.pow(2) - y.pow(2);
            if M3.contains(&(m % 60)) && m <= n {
                number_array[m as usize] = !number_array[m as usize];
            }
        }
    }

    let S: Vec<u64> = vec![1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59];
    let M4: Vec<u64> = (0..=((n as usize)/60)).map(|x| (x * 60) as u64).flat_map(|a| S.iter().map(move |b| a + b)).collect();
    for m in &M4 {
        if m.pow(2) > n {
            break;
        } else if *m != 1 {
            let mm = m.pow(2);
            if number_array[*m as usize] {
                for m2 in &M4 {
                    let c = mm * m2;
                    if c > n {
                        break;
                    } else {
                        number_array[c as usize] = false;
                    }
                }
            }
        }
    }

    let mut primes: Vec<u64> = vec![2, 3, 5];
    for (idx, x) in number_array.iter().enumerate() {
        if *x { primes.push(idx as u64); }
    }

    primes
}

pub fn vec_from_bounds(lower_bound: u64, upper_bound: u64) -> Vec<u64> {
    // create array of numbers from lower to upper
    (lower_bound..=upper_bound).collect()
}

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

    #[test]
    fn utils_atkin() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
                            41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
                            89, 97, 101, 103, 107, 109, 113, 127, 131,
                            137, 139, 149, 151, 157, 163, 167, 173, 179,
                            181, 191, 193, 197, 199, 211, 223, 227, 229,
                            233, 239, 241, 251, 257, 263, 269, 271, 277,
                            281, 283, 293, 307, 311, 313, 317, 331, 337,
                            347, 349, 353, 359, 367, 373, 379, 383, 389,
                            397, 401, 409, 419, 421, 431, 433, 439, 443,
                            449, 457, 461, 463, 467, 479, 487, 491, 499,
                            503, 509, 521, 523, 541, 547, 557, 563, 569,
                            571, 577, 587, 593, 599, 601, 607, 613, 617,
                            619, 631, 641, 643, 647, 653, 659, 661, 673,
                            677, 683, 691, 701, 709, 719, 727, 733, 739,
                            743, 751, 757, 761, 769, 773, 787, 797, 809,
                            811, 821, 823, 827, 829, 839, 853, 857, 859,
                            863, 877, 881, 883, 887, 907, 911, 919, 929,
                            937, 941, 947, 953, 967, 971, 977, 983, 991, 997];
        assert!(atkin_sieve(1000) == expected);
    }
}