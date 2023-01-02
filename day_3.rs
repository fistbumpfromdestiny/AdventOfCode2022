fn get_prio(char: &char) -> u32 {
    if char.is_uppercase() {
        *char as u32 - 38
    } else {
        *char as u32 - 96
    }
}

fn main() {
    let file = include_str!("input.txt");
    let lines: Vec<&str> = file.lines().collect();

    let mut score = 0;
    let mut score_part_2 = 0;

    for line in file.lines().map(|l| l.trim().split_at(l.len() / 2)) {
        for char in line.0.chars() {
            if line.1.contains(char) {
                score += get_prio(&char);
                break;
            }
        }
    }

    println!("score: {}", score);
    println!(
        "score part 2: {:?}",
        lines
            .chunks(3)
            .map(|chunk| {
                for char in chunk[0].chars() {
                    if chunk[1].contains(char) && chunk[2].contains(char) {
                        return get_prio(&char);
                    }
                }
                panic!("No badge found!");
            })
            .sum::<u32>()
    )
}
