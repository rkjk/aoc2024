use crate::utils::{read_input, bench};
use std::fmt::{write, Debug, Formatter, Result};
use std::cmp::PartialEq;
use std::ptr::eq;
use rayon::prelude::*;
use itertools::Itertools;

type NumType = u64;

#[derive(Debug)]
struct Equation {
    pub result: NumType,
    pub nums: Vec<NumType>
}
#[derive(Copy, Clone)]
enum Operator {
    Add,
    Mul,
    Concat,
    None
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Mul => write!(f, "*"),
            Operator::None => write!(f, " "),
            Operator::Concat => write!(f, "||"),
        }
    }
}

#[derive(Debug)]
struct Context {
    equations: Vec<Equation>,
    operator_permutations_1: Vec<Vec<Vec<Operator>>>,
    operator_permutations_2: Vec<Vec<Vec<Operator>>>,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let equations: Vec<Equation> = inp.into_iter().map(|s| {
            let t1: Vec<&str> = s.split(':').collect();
            let result = t1[0].parse::<NumType>().unwrap();
            let nums = t1[1]
                .trim()
                .split_whitespace()
                .into_iter()
                .map(|f| f.parse::<NumType>().unwrap())
                .collect();
            Equation {
                result: result,
                nums: nums
            }
        }).collect();
        let max_len = equations.iter().map(|eq| eq.nums.len()).max().unwrap();
        let mut operator_permutations_1 = vec![vec![vec![Operator::None]]];
        let mut operator_permutations_2 = vec![vec![vec![Operator::None]]];
        let enum_items_1 = [Operator::Add, Operator::Mul];
        let enum_items_2 = [Operator::Add, Operator::Mul, Operator::Concat]; 
        for length in 1..max_len {
            let permutations = std::iter::repeat(enum_items_1.into_iter())
                .take(length)
                .multi_cartesian_product()
                .collect_vec();
            operator_permutations_1.push(permutations);
            let permutations = std::iter::repeat(enum_items_2.into_iter())
            .take(length)
            .multi_cartesian_product()
            .collect_vec();
            operator_permutations_2.push(permutations);
        }

        Context {
            equations: equations,
            operator_permutations_1: operator_permutations_1,
            operator_permutations_2: operator_permutations_2
        }
    }

    fn concatenate(x: NumType, y: NumType) -> NumType {
        // Calculate the number of digits in y
        let y_digits = y.to_string().len() as u32;
    
        // Shift x to the left by the number of digits in y
        let shifted_x = x * 10_u64.pow(y_digits);
    
        // Add y to the shifted x
        shifted_x + y
    }

    fn compute(equation: &Equation, perm: &Vec<Operator>) -> bool {
        let mut s = equation.nums[0];
        for i in 0..perm.len() {
            match perm[i] {
                Operator::Add => s += equation.nums[i + 1],
                Operator::Mul => s *= equation.nums[i + 1],
                Operator::Concat => s = Context::concatenate(s, equation.nums[i + 1]),
                _ => panic!("Unknown Operator"),
            };
        }
        return s == equation.result;
    }

    fn helper(equation: &Equation, operator_permutations: &Vec<Vec<Vec<Operator>>>) -> NumType {
        let res = equation.result;
        let len = equation.nums.len();
        for p in &operator_permutations[len - 1] {
            if Context::compute(equation, p) {
                return res;
            }

        }
        0
    }

    pub fn part1(&self) -> NumType {
        self.equations.par_iter()
        .map(|eq| Context::helper(eq, &self.operator_permutations_1))
        .sum()
    }

    pub fn part2(&self) -> NumType {
        self.equations.par_iter()
        .map(|eq| Context::helper(eq, &self.operator_permutations_2))
        .sum()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc7 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc7/example").expect("couldn't read input - aoc7");
        let context = Context::new(text);
        let part1 = context.part1();
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc7/input").expect("couldn't read input - aoc7");
        let context = Context::new(text);
        let part1 = bench(|| context.part1(), Some("part2"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}