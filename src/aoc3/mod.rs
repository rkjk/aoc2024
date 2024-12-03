use crate::utils::read_input;
use regex::Regex;
use std::cmp::PartialEq;

#[derive(Debug)]
struct Context {
    memory: String
}

#[derive(Debug, PartialEq)]
enum Condition {
    Do,
    Dont
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        Context {
            memory: inp.join("")
        }
    }

    pub fn part1(&self) -> u32 {
        let mut sum = 0;
        let re = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();
        for cap in re.find_iter(&self.memory) {
            let s = &cap.as_str()[4..];
            let s = &s[..s.len() - 1];
            let nums: Vec<&str> = s.split(",").collect();
            let num1 = nums[0].parse::<u32>().unwrap();
            let num2 =  nums[1].parse::<u32>().unwrap();
            sum += num1 * num2;
        }
        sum
    }

    pub fn part2(&self) -> u32 {
        let mut sum = 0;
        let re1 = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();
        let re2 = Regex::new(r"don't\(\)|do\(\)").unwrap();
        let conditions: Vec<(Condition, usize)> = re2.find_iter(&self.memory).map(|v| {
            let s = v.as_str();
            return match s {
                "do()" => (Condition::Do, v.start()),
                "don't()" => (Condition::Dont, v.start()),
                _ => panic!("Unexpected input")
            }
        }).collect();
        let mut cur_ind = 0;
        for cap in re1.find_iter(&self.memory) {
            let s = &cap.as_str()[4..];
            let s = &s[..s.len() - 1];
            let nums: Vec<&str> = s.split(",").collect();
            let num1 = nums[0].parse::<u32>().unwrap();
            let num2 =  nums[1].parse::<u32>().unwrap();
            let start = cap.start();
            while cur_ind < conditions.len() && conditions[cur_ind].1 < start {
                cur_ind += 1;
            }
            if cur_ind == 0 || conditions[cur_ind - 1].0 == Condition::Do {
                sum += num1 * num2;
            }
        }
        sum
    }
}



#[cfg(test)]
mod aoc3 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc3/example").expect("couldn't read input - aoc3");
        let context = Context::new(text);
        println!("Part1: {:?}", context.part1());
        let text: Vec<String> = read_input("src/aoc3/example2").expect("couldn't read input - aoc3");
        let context = Context::new(text);
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc3/input").expect("couldn't read input - aoc3");
        let context = Context::new(text);
        println!("Part1: {:?}", context.part1());
        println!("Part2: {:?}", context.part2());
    }
}