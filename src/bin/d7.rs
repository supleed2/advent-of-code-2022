const INPUT: &str = include_str!("../input/d7.txt");

struct Dir {
    name: String,
    contents: Vec<Node>,
}

struct File {
    size: usize,
}

enum Node {
    Dir(Dir),
    File(File),
}

fn main() -> anyhow::Result<()> {
    let mut tree = Dir {
        name: "/".to_string(),
        contents: vec![],
    };
    let mut cwd = vec!["/".to_string()];
    for line in INPUT.lines() {
        let mut l = line.split(' ');
        match l.next() {
            Some("$") => match l.next() {
                Some("cd") => {
                    let dest = l.next().expect("cd must have destination");
                    match dest {
                        "/" => cwd.truncate(1),
                        ".." => {
                            cwd.pop();
                        }
                        _ => cwd.push(dest.to_string()),
                    }
                }
                Some("ls") => {}
                _ => unreachable!(),
            },
            Some("dir") => {
                let mut dir = &mut tree;
                for cd in &cwd[1..] {
                    dir = dir
                        .contents
                        .iter_mut()
                        .find_map(|n| {
                            if let Node::Dir(d) = n {
                                if &d.name == cd {
                                    return Some(d);
                                }
                            };
                            None
                        })
                        .expect("cd should be valid");
                }
                dir.contents.push(Node::Dir(Dir {
                    name: l.next().expect("dir must have a name").to_string(),
                    contents: vec![],
                }));
            }
            Some(size) => {
                let mut dir = &mut tree;
                for cd in &cwd[1..] {
                    dir = dir
                        .contents
                        .iter_mut()
                        .find_map(|n| {
                            if let Node::Dir(d) = n {
                                if &d.name == cd {
                                    return Some(d);
                                }
                            };
                            None
                        })
                        .expect("cd should be valid");
                }
                dir.contents.push(Node::File(File {
                    size: size.parse().expect("filesize must be unsigned int"),
                }));
            }
            None => unreachable!(),
        }
    }
    let mut dirs: Vec<(&str, usize)> = vec![];
    dir_size(&mut dirs, &tree);
    let p1: usize = dirs.iter().filter(|(_, s)| s <= &100000).map(|d| d.1).sum();
    println!("{p1}");
    let target = dirs
        .iter()
        .find(|(name, _)| name == &"/")
        .expect("root dir must have a size")
        .1
        - 40_000_000;
    dirs.sort_unstable_by_key(|k| k.1);
    let p2 = dirs
        .iter()
        .find(|(_, s)| s >= &target)
        .expect("a dir big enough must exist")
        .1;
    println!("{p2}");
    anyhow::Ok(())
}

fn dir_size<'a>(vec: &mut Vec<(&'a str, usize)>, dir: &'a Dir) -> usize {
    let size = dir
        .contents
        .iter()
        .map(|n| match n {
            Node::Dir(d) => dir_size(vec, d),
            Node::File(f) => f.size,
        })
        .sum();
    vec.push((dir.name.as_ref(), size));
    size
}
