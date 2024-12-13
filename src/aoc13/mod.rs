use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use rayon::prelude::*;
use nalgebra::{Matrix2, Vector2};

type Num = f64;

#[derive(Debug)]
struct Equation {
    mat: Matrix2<Num>,
    vec: Vector2<Num>
}


impl Equation {
    // Constructor method
    pub fn new(mat: Matrix2<Num>, vec: Vector2<Num>) -> Self {
        Equation { mat, vec }
    }

    // Method to solve the equation using matrix inversion
    pub fn solve(&self) -> Option<Vector2<Num>> {
        // Check if the matrix is invertible
        if self.mat.determinant() == 0.0 {
            //println!("Eq: {:?} Determinant zero", self);
            None
        } else {
            let inv_mat = self.mat.try_inverse().unwrap();
            let r = inv_mat * self.vec;
            //println!("Eq: {:?} solution {:?}", self, r);
            Some(inv_mat * self.vec)
        }
    }

    pub fn add(&mut self, val: Num) {
        self.vec[0] += val;
        self.vec[1] += val;
    }
}

#[derive(Debug)]
struct Context {
    equations: Vec<Equation>
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let mut eqs = vec![];
        for i in (0..inp.len()).step_by(4) {
            let buttona = Context::extract_number(&inp[i]);
            let buttonb = Context::extract_number(&inp[i + 1]);
            let mat = Matrix2::new(buttona.0, buttonb.0, buttona.1, buttonb.1);
            let res = Context::extract_result(&inp[i + 2]);
            let vec = Vector2::new(res.0, res.1);
            eqs.push(Equation::new(mat, vec));
        }
        Context {
            equations: eqs
        }
    }

    fn extract_number(input: &String) -> (Num, Num) {
        let numbers: Vec<&str> = input.split(&['+', ',']).filter(|s| s.trim().chars().all(char::is_numeric)).collect();
        let numbers: Vec<Num> = numbers.into_iter().map(|s| s.trim().parse::<Num>().unwrap()).collect();
        (numbers[0], numbers[1])
    }

    fn extract_result(input: &String) -> (Num, Num) {
        // Split the input string by '=' and ',' to get the numeric parts
        let parts: Vec<&str> = input.split(&['=', ',']).collect();
    
        // Filter out non-numeric parts and parse the numbers
        let mut numbers = parts.iter()
            .filter_map(|part| part.trim().parse::<Num>().ok())
            .collect::<Vec<Num>>();
    
        // Ensure we have exactly two numbers
        if numbers.len() != 2 {
            panic!("Invalid input format");
        }
    
        // Return the numbers as a tuple
        (numbers[0], numbers[1])
    }

    fn check_valid(n: Num) -> bool {
        //println!("num: {}, fract_abs: {}, round: {}", n, n.fract().abs(), n.round());
        let fractional_part = n.fract().abs();
        fractional_part < 1e-4 || fractional_part > 0.9999 && n.round() as usize <= 100
    }

    fn get_cost(v: &Vector2<Num>) -> usize {
        //println!("vector: {:?}", v);

        if Context::check_valid(v[0]) && Context::check_valid(v[1]) {
            //println!("valid");
            return v[0].round() as usize * 3 + v[1].round() as usize;
        }
        0
    }

    fn check_valid_part2(n: Num) -> bool {
        //println!("num: {}, fract_abs: {}, round: {}", n, n.fract().abs(), n.round());
        let fractional_part = n.fract().abs();
        fractional_part < 1e-2 || fractional_part > 0.99
    }

    fn get_cost_part2(v: &Vector2<Num>) -> usize {
        //println!("vector: {:?}", v);
        if Context::check_valid_part2(v[0]) && Context::check_valid_part2(v[1]) {
            //println!("valid");
            return v[0].round() as usize * 3 + v[1].round() as usize;
        }
        0
    }

    pub fn part1(&self) -> usize {
        self.equations.iter().map(|eq| {
            match eq.solve() {
                None => 0,
                Some(v) => Context::get_cost(&v),
            }
        }).sum()
    }

    pub fn part2(&mut self) -> usize {
        let val: Num = 10000000000000.0;
        self.equations.iter_mut().map(|eq| {
            eq.add(val);
            match eq.solve() {
                None => 0,
                Some(v) => Context::get_cost_part2(&v),
            }
        }).sum()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc13 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc13/example").expect("couldn't read input - aoc13");
        let mut context = Context::new(text);
        //println!("context: {:?}", context);
        let part1 = context.part1();
        println!("Example part1: {}", part1);
        //let (part1, area) = context.part1();
        //println!("Example1 Part1: {:?}", part1);
        println!("Example1 Part2: {:?}", context.part2());
    }

   #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc13/input").expect("couldn't read input - aoc13");
        let mut context = Context::new(text);
        let part1 = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}