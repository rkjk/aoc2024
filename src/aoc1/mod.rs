use crate::utils::read_input;
use std::collections::HashMap;

#[derive(Debug)]
struct Context {
    list1: Vec<i32>,
    list2: Vec<i32>
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let tmp: Vec<(i32, i32)> = inp.into_iter()
        .map(|s| { 
            let mut nums = s.as_str().trim().split_whitespace();
            let first = nums.next().unwrap().parse::<i32>().unwrap();
            let second = nums.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .collect();
        
        Context {
            list1: {
                let mut t: Vec<i32> = tmp.iter().map(|nums| nums.0).collect();
                t.sort();
                t
            },
            list2: {
                let mut t: Vec<i32> = tmp.iter().map(|nums| nums.1).collect();
                t.sort();
                t
            },
        }
    }

    pub fn part1(&self) -> u32 {
        let mut diff: u32 = 0;
        for i in 0..self.list1.len() {
            diff += (self.list1[i] - self.list2[i]).wrapping_abs() as u32;
        }
        diff
    }

    pub fn part2(&self) -> u32 {
        let mut map: HashMap<i32, u32> = HashMap::new();
        let mut sim: u32 = 0;
        for i in 0..self.list2.len() {
            map.entry(self.list2[i]).and_modify(|c| *c += 1).or_insert(1);
        }
        for i in 0..self.list1.len() {
            sim += match map.get(&self.list1[i]) {
                Some(x) => x * self.list1[i] as u32,
                None => 0
            };
        }
        sim
    }
}

#[cfg(test)]
mod aoc1 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc1/example").expect("couldn't read input - aoc1");
        let context = Context::new(text);
        println!("Part1: {:?}", context.part1());
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc1/input").expect("couldn't read input - aoc1");
        let context = Context::new(text);
        println!("Part1: {:?}", context.part1());
        println!("Part2: {:?}", context.part2());
    }
}