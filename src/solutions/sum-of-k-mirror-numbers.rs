// Category: algorithms
// Level: Hard
// Percent: 45.040577%

// A k-mirror number is a positive integer without leading zeros that reads the same both forward and backward in base-10 as well as in base-k.
//
//
// 	For example, 9 is a 2-mirror number. The representation of 9 in base-10 and base-2 are 9 and 1001 respectively, which read the same both forward and backward.
// 	On the contrary, 4 is not a 2-mirror number. The representation of 4 in base-2 is 100, which does not read the same both forward and backward.
//
//
// Given the base k and the number n, return the sum of the n smallest k-mirror numbers.
//
//
// Example 1:
//
// Input: k = 2, n = 5
// Output: 25
// Explanation:
// The 5 smallest 2-mirror numbers and their representations in base-2 are listed as follows:
//   base-10    base-2
//     1          1
//     3          11
//     5          101
//     7          111
//     9          1001
// Their sum = 1 + 3 + 5 + 7 + 9 = 25.
//
//
// Example 2:
//
// Input: k = 3, n = 7
// Output: 499
// Explanation:
// The 7 smallest 3-mirror numbers are and their representations in base-3 are listed as follows:
//   base-10    base-3
//     1          1
//     2          2
//     4          11
//     8          22
//     121        11111
//     151        12121
//     212        21212
// Their sum = 1 + 2 + 4 + 8 + 121 + 151 + 212 = 499.
//
//
// Example 3:
//
// Input: k = 7, n = 17
// Output: 20379000
// Explanation: The 17 smallest 7-mirror numbers are:
// 1, 2, 3, 4, 5, 6, 8, 121, 171, 242, 292, 16561, 65656, 2137312, 4602064, 6597956, 6958596
//
//
//
// Constraints:
//
//
// 	2 <= k <= 9
// 	1 <= n <= 30
//

#[allow(unused)]
struct Solution;

// @lc code=start
#[allow(unused)]
struct MirrorNumbersEvenDigits {
    prefix: i64,
}

impl MirrorNumbersEvenDigits {
    #[allow(unused)]
    fn new() -> Self {
        Self { prefix: 0 }
    }
}

impl Iterator for MirrorNumbersEvenDigits {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.prefix += 1;
        let (rev, pow) = reverse_digits(self.prefix);
        Some(self.prefix * 10i64.pow(pow) + rev)
    }
}

#[allow(unused)]
struct MirrorNumbersOddDigits {
    prefix: i64,
    middle: i64,
}

impl MirrorNumbersOddDigits {
    #[allow(unused)]
    fn new() -> Self {
        Self {
            prefix: 0,
            middle: 0,
        }
    }
}

impl Iterator for MirrorNumbersOddDigits {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.middle += 1;
        if self.middle == 10 {
            self.middle = 0;
            self.prefix += 1;
        }
        let (rev, pow) = reverse_digits(self.prefix);
        Some((self.prefix * 10 + self.middle) * 10i64.pow(pow) + rev)
    }
}

#[allow(unused)]
struct MirrorNumbers {
    iter_even: MirrorNumbersEvenDigits,
    iter_odd: MirrorNumbersOddDigits,
    number_even: i64,
    number_odd: i64,
}

impl MirrorNumbers {
    #[allow(unused)]
    fn new() -> Self {
        let mut iter_even = MirrorNumbersEvenDigits::new();
        let mut iter_odd = MirrorNumbersOddDigits::new();
        let number_even = iter_even.next().unwrap();
        let number_odd = iter_odd.next().unwrap();
        Self {
            iter_even: iter_even,
            iter_odd: iter_odd,
            number_even: number_even,
            number_odd: number_odd,
        }
    }
}

impl Iterator for MirrorNumbers {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let number_even = self.number_even;
        let number_odd = self.number_odd;
        if number_even < number_odd {
            self.number_even = self.iter_even.next().unwrap();
            Some(number_even)
        } else {
            self.number_odd = self.iter_odd.next().unwrap();
            Some(number_odd)
        }
    }
}

/// Check if number x is a k-mirror number
fn is_k_mirror(mut x: i64, k: i32) -> bool {
    let mut digits = vec![];
    while x > 0 {
        digits.push(x % k as i64);
        x = x / k as i64;
    }
    let half = digits.len() / 2;
    digits[..half]
        .iter()
        .zip(digits[digits.len() - half..].iter().rev())
        .all(|(&x, &y)| x == y)
}

/// Revert a 10-base integer
fn reverse_digits(mut num: i64) -> (i64, u32) {
    let mut rev = 0;
    let mut pow = 0;
    while num > 0 {
        rev = rev * 10 + num % 10;
        num /= 10;
        pow += 1;
    }
    (rev, pow)
}

#[allow(unused)]
impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        MirrorNumbers::new()
            .filter(|&x| is_k_mirror(x, k))
            .take(n as usize)
            .sum()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::k_mirror(2, 5), 25);
        assert_eq!(Solution::k_mirror(3, 7), 499);
        assert_eq!(Solution::k_mirror(7, 17), 20379000);
    }
}
