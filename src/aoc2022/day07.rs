//! # Day 7: No Space Left On Device
//!
//! [puzzle site](https://adventofcode.com/2022/day/7)

struct Dir {
    parent: Option<usize>,
    children: std::collections::HashMap<String, usize>,
    content: std::collections::HashMap<String, u32>,
    size: u32,
}

fn parse_commands(input: String) -> Vec<Dir> {
    // initialize file system with empty root directory
    let mut cwd = 0;
    let mut fs = vec![Dir {
        parent: None,
        children: std::collections::HashMap::new(),
        content: std::collections::HashMap::new(),
        size: 0,
    }];

    // loop over terminal output
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "$" {
            // change directory
            if words[1] == "cd" {
                cwd = match words[2] {
                    "/" => 0,
                    ".." => fs[cwd].parent.unwrap(),
                    dir => *fs[cwd].children.get(dir).unwrap(),
                }
            }
        } else if words[0] == "dir" {
            // add directory
            let name = words[1];
            if !fs[cwd].children.contains_key(name) {
                let index = fs.len();
                fs[cwd].children.insert(name.to_string(), index);
                fs.push(Dir {
                    parent: Some(cwd),
                    children: std::collections::HashMap::new(),
                    content: std::collections::HashMap::new(),
                    size: 0,
                });
            }
        } else {
            // add file and increase size of parent directories
            let name = words[1];
            if !fs[cwd].content.contains_key(name) {
                let size: u32 = words[0].parse().unwrap();
                fs[cwd].content.insert(name.to_string(), size);
                let mut index = cwd;
                loop {
                    fs[index].size += size;
                    match fs[index].parent {
                        Some(i) => index = i,
                        None => break,
                    }
                }
            }
        }
    }
    fs
}

/// Part 1
pub fn part1(input: String) -> crate::PuzzleResult {
    let fs = parse_commands(input);
    let mut sum = 0;
    for dir in fs {
        if dir.size <= 100000 {
            sum += dir.size;
        }
    }
    Ok(sum.to_string())
}

/// Part 2
pub fn part2(input: String) -> crate::PuzzleResult {
    let fs = parse_commands(input);
    let mut size = fs[0].size;
    let needed = size - 40000000;
    for dir in fs {
        if dir.size >= needed && dir.size < size {
            size = dir.size;
        }
    }
    Ok(size.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k"
    );

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "95437");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "24933642");
    }
}
