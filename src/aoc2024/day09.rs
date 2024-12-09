//! # Day 9: Disk Fragmenter
//!
//! The input is a sequence of digits that alternately describe a file or free
//! space size in memory blocks. The files are given an ID counting up from 0.
//! The task is to fill gaps in memory in a certain way and the solution is the
//! sum over all products of final block index and the file ID of its content.
//!
//! [puzzle site](https://adventofcode.com/2024/day09)

/// Part 1: Occupied blocks are moved from the end to the first free block until
/// no gaps remain
pub fn part1(input: String) -> crate::PuzzleResult {
    let numbers = input.chars().filter_map(|c| c.to_digit(10));
    let mut files = Vec::new();
    let mut space = Vec::new();
    for (id, len) in numbers.clone().step_by(2).enumerate() {
        files.push((id, len as u8));
    }
    for len in numbers.skip(1).step_by(2) {
        space.push(len as u8);
    }
    let mut insert_at = 1;
    let mut space_pointer = 0;
    while insert_at < files.len() {
        let (id, mut len) = files.pop().unwrap();
        while len >= space[space_pointer] {
            files.insert(insert_at.min(files.len()), (id, space[space_pointer]));
            len -= space[space_pointer];
            space[space_pointer] = 0;
            insert_at += 2;
            space_pointer += 1;
        }
        if len > 0 {
            files.insert(insert_at.min(files.len()), (id, len));
            space[space_pointer] -= len;
            insert_at += 1
        }
    }
    let mut checksum = 0;
    let mut position = 0;
    for (id, len) in files {
        for i in 0..len {
            checksum += (position + i as usize) * id;
        }
        position += len as usize;
    }
    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(&super::part1(INPUT.to_string()).unwrap(), "1928");
    }
}
