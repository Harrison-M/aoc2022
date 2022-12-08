//! Day 7

use std::collections::HashMap;

use util::*;

#[derive(Default)]
struct Directory<'a> {
    dirs: HashMap<&'a str, Directory<'a>>,
    files: HashMap<&'a str, usize>,
}

fn parse(input: &str) -> Result<Directory, Error> {
    let mut root = Directory::default();

    // Just avoiding dealing with references and pointers by just tracking our path
    let mut stack: Vec<&str> = vec![];
    let mut current = &mut root;

    for line in input.lines() {
        let split = line
            .rsplit_once(' ')
            .ok_or_else(|| anyhow!("Invalid input line: {}", line))?;

        match split {
            ("$ cd", "/") | ("$", "ls") => (),
            // Create displayed directory if it does not already exist
            ("dir", dir_name) => {
                current.dirs.entry(dir_name).or_default();
            }
            ("$ cd", "..") => {
                stack.pop();
                current = &mut root;
                for dir in stack.iter() {
                    current = current
                        .dirs
                        .get_mut(dir)
                        .context("Could not navigate stack")?;
                }
            }
            ("$ cd", dir_name) => {
                stack.push(dir_name);
                current = current.dirs.entry(dir_name).or_default();
            }
            (size, file_name) => {
                current
                    .files
                    .entry(file_name)
                    .or_insert(size.parse().context("Failed to parse file size")?);
            }
        };
    }

    Ok(root)
}

/// Return a list of directory sizes and a total file size for a directory
fn sizes(dir: &Directory) -> (Vec<usize>, usize) {
    let (nested_dir_sizes, file_sizes): (Vec<Vec<usize>>, Vec<usize>) =
        dir.dirs.values().map(sizes).unzip();

    let mut dir_sizes: Vec<usize> = nested_dir_sizes.into_iter().flatten().collect();
    let size_here = dir.files.values().sum::<usize>() + file_sizes.into_iter().sum::<usize>();
    dir_sizes.push(size_here);
    (dir_sizes, size_here)
}

fn part1(root: &Directory) -> usize {
    sizes(root)
        .0
        .into_iter()
        .filter(|size| *size <= 100000)
        .sum()
}

fn part2(root: &Directory) -> Option<usize> {
    let (dir_sizes, file_size) = sizes(root);
    let free_space = 70000000 - file_size;
    let mut candidates: Vec<usize> = dir_sizes
        .into_iter()
        .filter(|size| *size >= 30000000 - free_space)
        .collect();

    candidates.sort();
    candidates.first().copied()
}

fn main() -> Result<(), Error> {
    let input = read_stdin()?;
    let root = parse(&input)?;
    println!("Part 1: {}", part1(&root));
    println!(
        "Part 2: {}",
        part2(&root).context("Deletion candidate not found")?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("./sample");

    #[test]
    fn parsing() -> Result<(), Error> {
        let root = parse(SAMPLE)?;
        assert!(root.dirs.contains_key("a"));
        assert!(root.dirs.contains_key("d"));
        assert_eq!(*root.files.get("b.txt").context("No b.txt")?, 14848514);
        assert_eq!(*root.files.get("c.dat").context("No c.dat")?, 8504156);
        let subdir = root
            .dirs
            .get("a")
            .context("a vanished")?
            .dirs
            .get("e")
            .context("Missing e")?;
        assert_eq!(*subdir.files.get("i").context("No i")?, 584);
        assert_eq!(subdir.files.len(), 1);
        Ok(())
    }

    #[test]
    fn part1_example() -> Result<(), Error> {
        let root = parse(SAMPLE)?;
        assert_eq!(part1(&root), 95437);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Error> {
        let root = parse(SAMPLE)?;
        assert_eq!(part2(&root).context("Not found")?, 24933642);
        Ok(())
    }
}
