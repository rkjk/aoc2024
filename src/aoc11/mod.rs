use crate::utils::{read_input, bench};
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use rayon::prelude::*;

type Num = u64;

#[derive(Debug)]
struct Context {
    nums: Vec<Num>,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        Context {
            nums: inp[0]
                    .split_ascii_whitespace()
                    .map(|v| v.parse::<Num>().unwrap())
                    .collect(),
        }
    }

    pub fn split_number(n: &Num) -> Vec<Num> {
        let str_n = n.to_string();
        let len = str_n.len();
        let mid = len / 2;
    
        let left = str_n[..mid].parse::<Num>().unwrap();
        let right = str_n[mid..].parse::<Num>().unwrap();
    
        vec![left, right]
    }

    pub fn process_number(num: &Num) -> Vec<Num> {
        let num_digits = num.checked_ilog10().unwrap_or(0) + 1;
        //println!("Num digits: {}", num_digits);
        if *num == 0 {
            return vec![1];
        } else if num_digits % 2 == 0 {
            //let split_nums = self.split_number(num);
            //println!("Orig num: {}, Split nums: {:?}", num, split_nums);
            return Context::split_number(num);
        } else {
            return vec![num * 2024];
        }
    }

    pub fn part1(&mut self, blinks: usize) -> usize {
        let mut map: HashMap<Num, usize> = HashMap::new();
        for x in self.nums.iter() {
            map.insert(*x, 1);   
        }
        for i in 0..blinks {
            let mut new_map = HashMap::new();
            for (s, n) in map.iter() {
                for t in Context::process_number(s) {
                    *new_map.entry(t).or_insert(0) += n;
                }
            }
            map = new_map;
        }
        map.values().sum()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc11 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc11/example").expect("couldn't read input - aoc11");
        let mut context = Context::new(text);
        //println!("split test: {:?}", context.split_number(&2000));
        println!("context: {:?}", context);
        let part1 = context.part1(25);
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", context.part1(75));
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc11/input").expect("couldn't read input - aoc11");
        let mut context = Context::new(text);
        let part1 = bench(|| context.part1(25), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part1(75), Some("part2")));
    }
}