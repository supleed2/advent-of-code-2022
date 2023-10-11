const INPUT: &str = include_str!("../input/d2.txt");

fn main() -> anyhow::Result<()> {
    let score_p1 = INPUT
        .as_bytes()
        .chunks(4)
        .map(|l| play_points(l[0] as char, l[2] as char))
        .sum::<usize>();
    println!("{score_p1:?}");
    let score_p2 = INPUT
        .as_bytes()
        .chunks(4)
        .map(|l| result_points(l[0] as char, l[2] as char))
        .sum::<usize>();
    println!("{score_p2:?}");
    anyhow::Ok(())
}

fn play_points(opponent: char, player: char) -> usize {
    match (opponent, player) {
        ('A', 'X') => 4,
        ('A', 'Y') => 8,
        ('A', 'Z') => 3,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 7,
        ('C', 'Y') => 2,
        ('C', 'Z') => 6,
        _ => unreachable!(),
    }
}

fn result_points(opponent: char, result: char) -> usize {
    match (opponent, result) {
        ('A', 'X') => 3,
        ('A', 'Y') => 4,
        ('A', 'Z') => 8,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 2,
        ('C', 'Y') => 6,
        ('C', 'Z') => 7,
        _ => unreachable!(),
    }
}
