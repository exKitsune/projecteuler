/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

// I could use another way where I can go over each multiple and do (m * floor(n/m) - 1 + m) / 2 * m and subtract common multiples
pub fn solve(n: u64, multiples: &[u64]) -> u64 {
    let mut sum: u64 = 0;

    for i in 1..n {
        for &m in multiples {
            if i % &m == 0 { sum += i; break; }
        }
    }

    sum
}

pub fn answer() -> u64{
    test();

    const MULTIPLES: [u64; 2] = [3, 5];
    solve(1000, &MULTIPLES)
}

fn test() {
    const MULTIPLES: [u64; 2] = [3, 5];
    assert!(solve(10, &MULTIPLES) == 23);
}