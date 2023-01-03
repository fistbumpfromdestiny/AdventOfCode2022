fn subset(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
    a_start >= b_start && a_start <= b_end && a_end >= b_start && a_end <= b_end
}

fn overlap(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
    subset(a_start, a_end, b_start, b_end) || subset(b_start, b_end, a_start, a_end)
}

fn subset_part_2(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
    a_start >= b_start && a_start <= b_end || a_end >= b_start && a_end <= b_end
}

fn overlap_part_2(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
    subset_part_2(a_start, a_end, b_start, b_end) || subset_part_2(b_start, b_end, a_start, a_end)
}
fn main() {
    let file = include_str!("input.txt");
    let mut counter = 0;
    let mut counter_2 = 0;

    for line in file.trim().lines().map(|l| l.trim().split_once(',')) {
        let (values_a, values_b) = line.unwrap();

        let values_a = values_a.split_once('-').unwrap();
        let a_start = values_a.0.trim().parse::<i32>().unwrap();
        let a_end = values_a.1.trim().parse::<i32>().unwrap();

        let values_b = values_b.split_once('-').unwrap();
        let b_start = values_b.0.trim().parse::<i32>().unwrap();
        let b_end = values_b.1.trim().parse::<i32>().unwrap();

        if overlap(a_start, a_end, b_start, b_end) {
            counter += 1;
        }
        if overlap_part_2(a_start, a_end, b_start, b_end) {
            counter_2 += 1;
        }
    }
    println!("{}", counter);
    println!("{}", counter_2)
}
