const INPUT: &str = include_str!("../input/d5.txt");

fn main() -> anyhow::Result<()> {
    let (_, moves) = INPUT
        .split_once("\n\n")
        .expect("Expected 2 sections of input file");
    // TODO: Replace hardcoded starting point, parse values instead
    let input = vec![
        vec![], // To make vector 1-indexed
        vec!['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
        vec!['N', 'V', 'G', 'P', 'H', 'W', 'B'],
        vec!['F', 'W', 'B', 'J', 'G'],
        vec!['G', 'J', 'N', 'F', 'L', 'W', 'C', 'S'],
        vec!['W', 'J', 'L', 'T', 'P', 'M', 'S', 'H'],
        vec!['B', 'C', 'W', 'G', 'F', 'S'],
        vec!['H', 'T', 'P', 'M', 'Q', 'B', 'W'],
        vec!['F', 'S', 'W', 'T'],
        vec!['N', 'C', 'R'],
    ];
    let mut p1c = input.clone();
    let p1m = moves
        .split('\n')
        .map(Move::from);
    for mv in p1m {
        for _ in 0..mv.0 {
            let item = p1c[mv.1].pop().expect("All moves should be valid");
            p1c[mv.2].push(item);
        }
    }
    let p1 = p1c
        .iter()
        .filter_map(|v| v.last())
        .collect::<String>();
    println!("{p1}");
    let mut p2c = input;
    let p2m = moves
        .split('\n')
        .map(Move::from);
    for mv in p2m {
        let mut temp = vec![];
        for _ in 0..mv.0 {
            temp.push(p2c[mv.1].pop().expect("All moves should be valid"));
        }
        for _ in 0..mv.0 {
            p2c[mv.2].push(temp.pop().expect("All moves should be valid"))
        }
    }
    let p2 = p2c
        .iter()
        .filter_map(|v| v.last())
        .collect::<String>();
    println!("{p2}");
    anyhow::Ok(())
}

#[derive(Debug)]
struct Move(usize, usize, usize);

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        let l = line
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();
        assert_eq!(l.len(), 3, "Expected 3 parts to move, line: {line}");
        Move(l[0], l[1], l[2])
    }
}
