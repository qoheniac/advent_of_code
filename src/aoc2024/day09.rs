//! # Day 9: Disk Fragmenter
//!
//! The input is a sequence of digits that alternately describe a file or free
//! space size in memory blocks. The files are given an ID counting up from 0.
//! The task is to fill gaps in memory in a certain way and the solution is the
//! sum over all products of final block index and the file ID of its content.
//!
//! [puzzle site](https://adventofcode.com/2024/day09)

#[derive(Clone, Copy)]
enum Section {
    Space(u8),
    File(usize, u8),
}
use Section::*;

fn parse(input: String) -> (Vec<Section>, usize) {
    let mut sections = Vec::new();
    let mut number_of_files = 0;
    for (i, len) in input.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        if len > 0 {
            let len = len as u8;
            if i % 2 == 0 {
                sections.push(File(i / 2, len));
                number_of_files += 1;
            } else {
                sections.push(Space(len));
            }
        }
    }
    (sections, number_of_files)
}

fn checksum(sections: Vec<Section>) -> usize {
    let mut checksum = 0;
    let mut position = 0;
    for section in sections {
        position += match section {
            File(id, len) => {
                let len = len as usize;
                checksum += (len * position + (len - 1) * len / 2) * id;
                len
            }
            Space(len) => len as usize,
        };
    }
    checksum
}

/// Part 1: Occupied blocks are moved from the end to the first free block until
/// no gaps remain
pub fn part1(input: String) -> crate::PuzzleResult {
    let (mut sections, _) = parse(input);
    'move_file: loop {
        let (id, mut len) = loop {
            match sections.pop().unwrap() {
                File(id, len) => break (id, len),
                Space(_) => continue,
            }
        };
        'fill_space: while len > 0 {
            for insert_at in 0..sections.len() {
                if let Space(space) = sections[insert_at] {
                    if space <= len {
                        sections[insert_at] = File(id, space);
                        len -= space;
                    } else {
                        sections[insert_at] = Space(space - len);
                        sections.insert(insert_at, File(id, len));
                        len = 0;
                    }
                    continue 'fill_space;
                }
            }
            sections.push(File(id, len));
            break 'move_file;
        }
    }
    Ok(checksum(sections).to_string())
}

/// Part 2: Files are moved in order of decreasing ID to the first gap that fits
/// the file
pub fn part2(input: String) -> crate::PuzzleResult {
    let (mut sections, number_of_files) = parse(input);
    for id in (0..number_of_files).rev() {
        let (old_index, len) = (sections.iter().copied().enumerate())
            .find_map(|(index, section)| match section {
                File(other_id, len) if other_id == id => Some((index, len)),
                _ => None,
            })
            .unwrap();
        if let Some((new_index, space)) = (sections[..old_index].iter().copied().enumerate())
            .find_map(|(index, section)| match section {
                Space(space) if space >= len => Some((index, space)),
                _ => None,
            })
        {
            (sections[new_index], sections[old_index]) = (sections[old_index], Space(len));
            if space > len {
                sections.insert(new_index + 1, Space(space - len));
            }
        }
    }
    Ok(checksum(sections).to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "1928");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&super::part2(INPUT.to_string()).unwrap(), "2858");
    }
}
