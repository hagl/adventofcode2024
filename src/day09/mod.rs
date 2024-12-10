use num::integer::gcd;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum DiskSection {
    File { id: i64, start: i64, length: i64 },
    Free { start: i64, length: i64 },
}

#[derive(Debug)]
struct File2 {
    id: i64,
    start: i64,
    length: i64,
}

#[derive(Debug)]
struct Free2 {
    start: i64,
    length: i64,
}

fn checksum(id: i64, start: i64, length: i64) -> i64 {
    id * (length * start + (length * (length - 1) / 2))
}

fn task1(array: &Vec<DiskSection>) -> i64 {
    let mut i: usize = 0;
    let mut end: usize = array.len();
    let mut acc: i64 = 0;
    let mut last: Option<(i64, i64)> = None;
    let mut last_start = 0;
    while i < end {
        let current = &array[i];
        i += 1;
        match current {
            DiskSection::File { id, start, length } => {
                acc += checksum(*id, *start, *length);
                last_start = start + length;
            }
            DiskSection::Free { start, length } => {
                let mut start = *start;
                last_start = start;
                let mut rest: i64 = *length;
                while rest > 0 && (last.is_some() || end > i) {
                    if let Some((last_id, last_length)) = last {
                        if last_length == rest {
                            acc += checksum(last_id, start, rest);
                            last = None;
                            start = start + rest;
                            last_start = start;
                            rest = 0;
                        } else if last_length > rest {
                            acc += checksum(last_id, start, rest);
                            last = Some((last_id, last_length - rest));
                            start += rest;
                            last_start = start;
                            rest = 0;
                        } else {
                            acc += checksum(last_id, start, last_length);
                            last = None;
                            start += last_length;
                            last_start = start;
                            rest = rest - last_length;
                        }
                    } else {
                        end = end - 1;
                        match array[end] {
                            DiskSection::Free {
                                start: _,
                                length: _,
                            } => {}
                            DiskSection::File {
                                id,
                                start: _,
                                length,
                            } => {
                                last = Some((id, length));
                            }
                        }
                    }
                }
            }
        }
    }
    if let Some((last_id, last_length)) = last {
        acc += checksum(last_id, last_start, last_length);
    }
    acc
}

fn task2(array: &Vec<DiskSection>) -> i64 {
    let mut frees: Vec<Free2> = array
        .iter()
        .filter_map(|ds| match ds {
            DiskSection::Free { start, length } => Some(Free2 {
                start: *start,
                length: *length,
            }),
            _ => None,
        })
        .collect();

    let mut files: Vec<File2> = array
        .iter()
        .filter_map(|ds| match ds {
            DiskSection::File { id, start, length } => Some(File2 {
                id: *id,
                start: *start,
                length: *length,
            }),
            _ => None,
        })
        .collect();
    files.reverse();

    let mut acc: i64 = 0;

    // No gaps greater than limit
    let mut limit: i64 = 9;
    for File2 { id, start, length } in files {
        if length > limit {
            acc += checksum(id, start, length);
        } else {
            match frees.iter_mut().find(
                |Free2 {
                     start: free_start,
                     length: free_length,
                 }| start < *free_start || length <= *free_length,
            ) {
                None => {
                    limit = length;
                    acc += checksum(id, start, length);
                }
                Some(Free2 {
                    start: free_start,
                    length: free_length,
                }) => {
                    if (start < *free_start) {
                        limit = length;
                        acc += checksum(id, start, length);
                    } else {
                        acc += checksum(id, *free_start, length);
                        *free_start += length;
                        *free_length -= length;
                    }
                }
            }
        }
    }

    acc
}

pub fn solve() -> String {
    let contents =
        fs::read_to_string("data/day09/input.txt").expect("Should have been able to read the file");
    // fs::read_to_string("data/day09/ex.txt").expect("Should have been able to read the file");
    // let mut result: Vec<DiskSection> = vec![];
    let array: Vec<DiskSection> = contents
        .split("\n")
        .map(|s| s.to_string())
        .next()
        .unwrap()
        .chars()
        .fold((vec![], 0, 0, true), |(mut acc, id, start, file), el| {
            let length: i64 = el.to_digit(10).unwrap().try_into().unwrap();
            if file {
                acc.push(DiskSection::File { id, start, length });
                (acc, id + 1, start + length, false)
            } else {
                acc.push(DiskSection::Free { start, length });
                (acc, id, start + length, true)
            }
        })
        .0;
    // println!("Parsed {:?}", array);

    format!("Task1: {}\nTask2: {}", task1(&array), task2(&array))
}
