const INPUT: &str = include_str!("../input/d4.txt");

fn main() -> anyhow::Result<()> {
    let p1 = INPUT
        .split('\n')
        .map(Pair::from)
        .filter(ranges_contain_each_other)
        .count();
    println!("{p1}");
    let p2 = INPUT
        .split('\n')
        .map(Pair::from)
        .filter(ranges_overlap)
        .count();
    println!("{p2}");
    anyhow::Ok(())
}

struct Pair(usize, usize, usize, usize);

impl From<&str> for Pair {
    fn from(line: &str) -> Self {
        let v = line
            .split(['-', ','])
            .map(|s| {
                s.parse::<usize>()
                    .expect("Failed to parse section ID as usize")
            })
            .collect::<Vec<_>>();
        assert_eq!(v.len(), 4, "Expected 4 section IDs, line: {line}");
        Pair(v[0], v[1], v[2], v[3])
    }
}

fn ranges_contain_each_other(pair: &Pair) -> bool {
    ((pair.0 <= pair.2)
        && (pair.3 <= pair.1))
    || ((pair.2 <= pair.0)
        && (pair.1 <= pair.3))
}

fn ranges_overlap(pair: &Pair) -> bool {
    !((pair.1 < pair.2) || (pair.3 < pair.0))
}