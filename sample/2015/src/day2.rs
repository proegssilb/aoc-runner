use aoc_zen_runner_macros::{solver, generator, aoc, aoc_case};

type Gift = (u32, u32, u32);

#[aoc(2015, day02)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<Gift> {
        input
            .lines()
            .map(|l| {
                let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
                (
                    gift.next().unwrap(),
                    gift.next().unwrap(),
                    gift.next().unwrap(),
                )
            }).collect()
    }

    #[solver(part1, dotchain)]
    pub fn solve_part1(input: Vec<Gift>) -> u32 {
        input
            .iter()
            .map(|&(l, w, h)| {
                let (s1, s2) = smallest_side((l, w, h));

                2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
            }).sum()
    }

    #[solver(part1, for_loop)]
    pub fn solve_part1_for(input: Vec<Gift>) -> u32 {
        let mut sum = 0;

        for (l, w, h) in input {
            let (s1, s2) = smallest_side((l, w, h));

            sum += 2 * l * w + 2 * w * h + 2 * h * l + s1 * s2;
        }

        sum
    }

    #[solver(part2, dotchain)]
    pub fn solve_part2(input: Vec<Gift>) -> u32 {
        input
            .iter()
            .map(|&(l, w, h)| {
                let (s1, s2) = smallest_side((l, w, h));

                (s1 + s2) * 2 + l * w * h
            }).sum()
    }

    fn smallest_side((l, w, h): Gift) -> (u32, u32) {
        let mut vec = vec![l, w, h];
        vec.sort();

        (vec[0], vec[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[aoc_case(58u32, 34u32)]
    const input1: &str = "2x3x4";

    #[aoc_case(43u32, 14u32)]
    const input2: &str = "1x1x10";
}