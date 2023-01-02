fn part_a(file: &str) -> u32 {
    let mut score = 0;

    for line in file.trim().lines().map(|l| l.trim().split_once(' ')) {
        score += match line {
            Some(("A", "X")) => 1 + 3,
            Some(("B", "X")) => 1 + 0,
            Some(("C", "X")) => 1 + 6,

            Some(("A", "Y")) => 2 + 6,
            Some(("B", "Y")) => 2 + 3,
            Some(("C", "Y")) => 2 + 0,

            Some(("A", "Z")) => 3 + 0,
            Some(("B", "Z")) => 3 + 6,
            Some(("C", "Z")) => 3 + 3,

            _ => 0,
        }
    }
    score
}

fn part_b(file: &str) -> u32 {
    let mut score = 0;

    for line in file.trim().lines().map(|l| l.trim().split_once(' ')) {
        score += match line {
            Some(("A", "X")) => 3 + 0,
            Some(("B", "X")) => 1 + 0,
            Some(("C", "X")) => 2 + 0,

            Some(("A", "Y")) => 1 + 3,
            Some(("B", "Y")) => 2 + 3,
            Some(("C", "Y")) => 3 + 3,

            Some(("A", "Z")) => 2 + 6,
            Some(("B", "Z")) => 3 + 6,
            Some(("C", "Z")) => 1 + 6,

            _ => 0,
        }
    }
    score
}
fn main() {
    let file = include_str!("../input.txt");
    let score = part_a(file);
    println!("Total score, part 1: {}", score);
    let score = part_b(file);
    println!("Total score, part 2: {}", score);
}
