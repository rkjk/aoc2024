use crate::utils::{read_input, bench};
use std::fmt::{write, Debug, Formatter, Result};
use rayon::prelude::*;

type Num = usize;

#[derive(Debug, Copy, Clone)]
struct Block {
    pub start_index: Num,
    pub length: Num,
    pub file_index: Num,
}


#[derive(Debug)]
struct Context {
    files: Vec<Block>,
    free: Vec<Block>,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let mut files = vec![];
        let mut free = vec![];
        let mut tot_length = 0;
        for (i, c) in inp[0].chars().enumerate() {
            let l = c.to_digit(10).unwrap() as Num;
            match i % 2 == 0 {
                true => {
                    if l == 0 {
                        panic!("zero sized file not supported");
                    }
                    let f = Block {
                        start_index: tot_length,
                        length: l,
                        file_index: files.len(),
                    };
                    files.push(f)
                },
                false => {
                    if l == 0 {
                        continue;
                    }
                    let f = Block {
                        start_index: tot_length,
                        length: l,
                        file_index: free.len(),
                    };
                    free.push(f);
                }
            };
            tot_length += l;
        }
        Context {
            files: files,
            free: free
        }
    }

    fn compute_checksum(files: &Vec<Block>) -> Num {
        files.iter().map(|f| {
            let mut sum = 0;
            for i in f.start_index..(f.start_index + f.length) {
                sum += i * f.file_index;
            }
            sum
        }).sum()
    }

    pub fn part1(&self) -> Num {
        let mut new_files: Vec<Block> = vec![];

        let mut files = self.files.clone();
        let mut free = self.free.clone();
        free.reverse();

        loop {
            // Get last file
            if files.is_empty() || free.is_empty() {
                break;
            }
            let mut cur_file = files.pop().unwrap();
            // Get last free (reversed, so from the beginning)
            let mut cur_free = free.pop().unwrap();
            // if start_index of file < free, break

            //println!("cur_file: {:?}", cur_file);
            //println!("cur free: {:?}", cur_free);
            if cur_file.start_index <= cur_free.start_index {
               //println!("Free space exhausted: break");
                files.push(cur_file);
                break;
            }
            // if length of file < length of free,
            //      change the start_index of the file and add to new_files
            //      reduce cur_file_ind
            //      modify the free length ->
            // if length of file = length of free,
            //      change the start_index of the file and add to new_files
            //      pop from free
            if cur_file.length <= cur_free.length {
                //println!("File fits in free space");
                let newf = Block {
                    start_index: cur_free.start_index,
                    length: cur_file.length,
                    file_index: cur_file.file_index
                };
                new_files.push(newf);
                if cur_file.length < cur_free.length {
                    cur_free.length -= cur_file.length;
                    cur_free.start_index += cur_file.length;
                    free.push(cur_free);
                }
            } 
            // if length of file > length of free,
            //      add a new file with this length and start_index of the free
            //      Change the length of file to be length - length-of-free
            //      pop from free
            else {
                //println!("File too big free space");
                let newf = Block {
                    start_index: cur_free.start_index,
                    length: cur_free.length,
                    file_index: cur_file.file_index
                };
                new_files.push(newf);
                cur_file.length -= cur_free.length;
                files.push(cur_file);
            }
        }
        files.extend(new_files);
        Context::compute_checksum(&files)
    }

    pub fn part2(&self) -> Num {
        let mut files = self.files.clone();
        let mut free = self.free.clone();

        for i in (0..files.len()).rev() {
            let mut cur_start = files[i].start_index;
            let mut picked_slot = Num::MAX;
            for j in 0..free.len() {
                if files[i].length <= free[j].length && cur_start > free[j].start_index {
                    cur_start = free[j].start_index;
                    picked_slot = j;
                }
            }
            // Slot has been picked
            if picked_slot != Num::MAX {
                files[i].start_index = cur_start;
                let remain_free = free[picked_slot].length - files[i].length;
                free[picked_slot].length = 0;
                if remain_free > 0 {
                    free.push(Block {
                        start_index: cur_start + self.files[i].length,
                        length: remain_free,
                        file_index: free[picked_slot].file_index
                    })
                }
            }
        }
        Context::compute_checksum(&files)
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc9 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc9/example").expect("couldn't read input - aoc9");
        let context = Context::new(text);
        //println!("context: {:?}", context);
        let part1 = context.part1();
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc9/input").expect("couldn't read input - aoc9");
        let context = Context::new(text);
        let part1 = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}