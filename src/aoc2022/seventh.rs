use std::collections::HashMap;

use crate::aoc2022::Solution;

#[derive(Default)]
struct Dir {
    parent: Option<usize>,
    children: HashMap<String, usize>, // all dir names are unique, so we can collet all inside flat hashmap.
    file_size: u64,
}

struct Fs {
    dirs: Vec<Dir>,
    cwd: usize,
}

impl Fs {
    fn new() -> Self {
        // 0 is root folder
        Self {
            dirs: vec![Dir::default()],
            cwd: 0,
        }
    }

    fn cd(&mut self, name: &str) {
        self.cwd = match name {
            "/" => 0,
            ".." => self.dirs[self.cwd].parent.unwrap(),
            _ => self.dirs[self.cwd].children[name],
        };
    }

    fn mkdir(&mut self, name: &str) {
        let id = self.dirs.len();
        self.dirs.push(Dir {
            parent: Some(self.cwd),
            ..Default::default()
        });
        self.dirs[self.cwd].children.insert(name.into(), id);
    }

    fn touch(&mut self, size: u64) {
        self.dirs[self.cwd].file_size += size;
    }

    fn parse(input: &str) -> Self {
        let mut fs = Fs::new();
        let mut lines = input.trim().lines().map(|s| s.trim()).peekable();
        lines.next(); // ingnore `$ cd /`

        while let Some(line) = lines.next() {
            if line.starts_with("$ cd") {
                let arg = &line[5..];
                fs.cd(arg);
            } else if line == "$ ls" {
                while let Some(&next) = lines.peek() {
                    // next instruction - leave
                    if next.starts_with('$') {
                        break;
                    }
                    let entry = lines.next().unwrap();
                    // dir
                    if entry.starts_with("dir") {
                        let arg = &entry[4..];
                        fs.mkdir(arg);
                    // file
                    } else {
                        let (size, _) = entry.split_once(' ').unwrap();
                        fs.touch(size.parse().unwrap());
                    }
                }
            }
        }

        fs
    }

    fn dir_sizes(&self) -> Vec<u64> {
        // collect files in folder
        let mut sizes: Vec<u64> = self.dirs.iter().map(|d| d.file_size).collect();
        // collect dirs - need to walk backwards from children folders to parents
        for i in (1..self.dirs.len()).rev() {
            let p = self.dirs[i].parent.unwrap();
            sizes[p] += sizes[i];
        }
        sizes
    }
}

pub struct SeventhSolution;

impl Solution for SeventhSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let fs = Fs::parse(input);
        fs.dir_sizes()
            .iter()
            .filter(|&&s| s < 100000)
            .sum::<u64>()
            .to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let fs = Fs::parse(input);
        let mut sizes = fs.dir_sizes();
        let free = 70000000 - sizes[0];
        sizes.sort();
        for &sz in &sizes {
            if free + sz >= 30000000 {
                return sz.to_string();
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
"#;

    #[test]
    fn day07_part1_example1() {
        let expect = "95437";
        let actual = SeventhSolution::part1(INPUT);
        assert_eq!(expect, actual);
    }

    #[test]
    fn day07_part2_example1() {
        let expect = "24933642";
        let actual = SeventhSolution::part2(INPUT);
        assert_eq!(expect, actual);
    }
}
