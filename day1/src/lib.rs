use std::fs::read_to_string;

pub fn part_a_solution(file_path: &str) -> u32 {
    let input = read_to_string(file_path).unwrap();

    let highest_sum = input
        .split("\n\n")
        .map(|group| -> u32 {
            group
                .lines()
                .map(|string| string.parse::<u32>().unwrap())
                .sum()
        })
        .max();

    highest_sum.unwrap()
}

pub fn part_b_solution(file_path: &str) -> u32 {
    let input = read_to_string(file_path).unwrap();

    let mut summed_calories: Vec<u32> = input
        .split("\n\n")
        .map(|group| -> u32 {
            group
                .lines()
                .map(|string| string.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    summed_calories.sort_by(|a, b| b.cmp(a));
    summed_calories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_test() {
        let result = part_a_solution("src/input.txt");
        dbg!(&result);
        // assert_eq!(result, 24000);
    }

    #[test]
    fn part_b_test() {
        let result = part_b_solution("src/input.txt");
        dbg!(&result);
        // assert_eq!(result, 45000);
    }
}
