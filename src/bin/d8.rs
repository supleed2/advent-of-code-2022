const INPUT: &str = include_str!("../input/d8.txt");
const N: usize = 99;

const fn gen_input<const N: usize>(input: &str) -> [[u8;N];N] {
    let mut out = [[0u8;N];N];
    let mut i = 0;
    let mut j = 0;
    while i < N {
        while j < N {
            out[i][j] = input.as_bytes()[i * (N + 1) + j] - 48;
            j += 1;
        }
        j = 0;
        i += 1;
    }
    out
}

fn main() -> anyhow::Result<()> {
    let trees: [[u8; N]; N] = gen_input::<N>(INPUT);
    let p1 = count_visible(&trees);
    println!("{p1}");
    let p2 = max_scenic_score(&trees);
    println!("{p2}");
    anyhow::Ok(())
}

fn count_visible(trees: &[[u8;N];N]) -> usize {
    let mut visible = [[false; N];N];
    // left -> right
    for i in 0..N {
        let mut bar = 0u8;
        for j in 0..N {
            if trees[i][j] >= bar {
                bar = trees[i][j] + 1;
                visible[i][j] = true;
            }
        }
    }
    // right -> left
    for i in 0..N {
        let mut bar = 0u8;
        for j in (0..N).rev() {
            if trees[i][j] >= bar {
                bar = trees[i][j] + 1;
                visible[i][j] = true;
            }
        }
    }
    // top -> bottom
    for j in 0..N {
        let mut bar = 0u8;
        for i in 0..N {
            if trees[i][j] >= bar {
                bar = trees[i][j] + 1;
                visible[i][j] = true;
            }
        }
    }
    // bottom -> top
    for j in 0..N {
        let mut bar = 0u8;
        for i in (0..N).rev() {
            if trees[i][j] >= bar {
                bar = trees[i][j] + 1;
                visible[i][j] = true;
            }
        }
    }
    visible.iter().flat_map(|r| r.iter()).filter(|v| **v).count()
}

fn max_scenic_score(trees: &[[u8;N];N]) -> usize {
    let mut max_score = 0usize;
    for i in 0..N {
        for j in 0..N {
            let mut scores = [0usize;4];
            let mut max = 0u8;
            // left
            for jt in (0..j).rev() {
                scores[0] += 1;
                if trees[i][jt] >= max {
                    max = trees[i][jt];
                }
                if max >= trees[i][j] {
                    break;
                }
            }
            max = 0;
            // right
            for jt in (j+1)..N {
                scores[1] += 1;
                if trees[i][jt] >= max {
                    max = trees[i][jt];
                }
                if max >= trees[i][j] {
                    break;
                }
            }
            max = 0;
            // up
            for it in (0..i).rev() {
                scores[2] += 1;
                if trees[it][j] >= max {
                    max = trees[it][j];
                }
                if max >= trees[i][j] {
                    break;
                }
            }
            max = 0;
            // down
            for it in (i+1)..N {
                scores[3] += 1;
                if trees[it][j] >= max {
                    max = trees[it][j];
                }
                if max >= trees[i][j] {
                    break;
                }
            }
            let score = scores.iter().product();
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}
