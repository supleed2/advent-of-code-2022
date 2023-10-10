const INPUT: &str = include_str!("../input/d1p1.txt");

fn main() -> anyhow::Result<()> {
    let mut elves = INPUT
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|s| s.parse::<usize>().expect("Failed to parse input"))
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    let max = elves.iter().max();
    println!("max: {max:?}");
    elves.sort_unstable_by(|l, r| r.cmp(l));
    let top3 = elves.iter().take(3).sum::<usize>();
    println!("top3: {top3:?}");
    anyhow::Ok(())
}
