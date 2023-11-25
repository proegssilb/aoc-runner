use crypto::digest::Digest;
use crypto::md5::Md5;
use std::u32;
use aoc_zen_runner_macros::{aoc, solution};

#[aoc(2015, day04)]
pub mod solutions {
    use super::*;
    
    #[solution(part1, crypto_md5)]
    pub fn part1(secret: &str) -> u32 {
        solver(secret, |hash| hash[0..2] == [0; 2] && (hash[2] & 0xF0) == 0)
    }

    #[solution(part2, crypto_md5)]
    pub fn part2(secret: &str) -> u32 {
        solver(secret, |hash| hash[0..3] == [0; 3])
    }

    fn solver(secret: &str, is_valid: impl Fn(&[u8; 16]) -> bool) -> u32 {
        let mut hash = [0; 16];

        let mut hasher = Md5::new();
        hasher.input_str(secret);

        (1..=u32::MAX)
            .map(|i| {
                let mut hasher = hasher;
                hasher.input_str(&i.to_string());
                hasher.result(&mut hash);

                (i, is_valid(&hash))
            }).find(|&(_, b)| b)
            .map(|(i, _)| i)
            .expect("result is bigger than u32")
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;

    #[test]
    // If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043
    // starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    fn example1() {
        assert_eq!(part1("abcdef"), 609043);
    }

    #[test]
    // If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash
    // starting with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like
    // 000006136ef....
    fn example2() {
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
