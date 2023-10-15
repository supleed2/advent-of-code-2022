use itertools::Itertools;

const INPUT: &str = include_str!("../input/d6.txt");

fn main() -> anyhow::Result<()> {
    let p1 = INPUT
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|w| w.1.iter().all_unique())
        .expect("Input should be valid")
        .0 + 4;
    println!("{p1}");
    let p2 = INPUT
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|w| w.1.iter().all_unique())
        .expect("Input should be valid")
        .0 + 14;
    println!("{p2}");
    anyhow::Ok(())
}
