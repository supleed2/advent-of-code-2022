use itertools::Itertools;

const INPUT: &str = include_str!("../input/d3.txt");

fn main() -> anyhow::Result<()> {
    let p1 = INPUT
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(l, r)| {
            l.chars().find(|ch| {
                r.contains(*ch)
            }).expect("No common characters in rucksack")
        })
        .map(item_priority)
        .sum::<usize>();
    println!("{p1}");
    let p2 = INPUT
        .lines()
        .batching(|it| {
            it.next_tuple::<(&str,&str,&str)>()
        })
        .map(|(a, b, c)| {
            a.chars()
                .filter(|&ch| b.contains(ch))
                .find(|&ch| c.contains(ch))
                .expect("No common characters in 3 rucksacks")
        })
        .map(item_priority)
        .sum::<usize>();
    println!("{p2:?}");
    anyhow::Ok(())
}

fn item_priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 96,
        'A'..='Z' => item as usize - 38,
        _ => unreachable!(),
    }
}
