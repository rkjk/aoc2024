use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::hash::Hash;
use std::ops::Index;
use std::usize;
use rayon::prelude::*;

type Num = usize;

type Pos = (Num, Num);

#[derive(Debug)]
struct Context {
    bytes: Vec<Pos>,
    r: usize,
    c: usize
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    pos: Pos,
    c: Num,
}

impl State {
    pub fn new(pos: Pos, c: Num) -> Self {
        State {
            pos,
            c
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.c.cmp(&self.c)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Context {
    pub fn new(inp: Vec<String>, r: usize, c: usize) -> Context {
        let val: Vec<Pos> = inp.into_iter().map(|s| {
            let tmp: Vec<Num> = s.trim().split(",").map(|f| f.parse::<Num>().unwrap()).collect();
            (tmp[0], tmp[1])
        }).collect();
        Context {
            bytes: val,
            r: r,
            c: c
        }
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut v = vec![];
        if pos.0 > 0 {
            v.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.r - 1 {
            v.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            v.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.c - 1 {
            v.push((pos.0, pos.1 + 1));
        }
        v
    }

    pub fn part1(&self, upto_len: usize) -> usize {
        let mut matrix: Vec<Vec<usize>> = vec![vec![usize::MAX - 1; self.c]; self.r];
        for k in 0..upto_len {
            matrix[self.bytes[k].0][self.bytes[k].1] = usize::MAX;
        }
        let mut q = BinaryHeap::new();
        q.push(State::new((0, 0), 0));
        let mut min_steps = Num::MAX;

        let mut c = 0;
        while !q.is_empty() {
            let State {pos: cur, c: steps} = q.pop().unwrap();
            if matrix[cur.0][cur.1] == steps {
                continue;
            }
            matrix[cur.0][cur.1] = steps;
            if cur.0 == self.r - 1 && cur.1 == self.c - 1 {
                min_steps = min_steps.min(steps);
                continue;
            }
            let ns = self.neighbors(cur);
            for (n1, n2) in ns {
                if matrix[n1][n2] != usize::MAX && matrix[n1][n2] > steps + 1 {
                    q.push(State::new((n1, n2), steps + 1));
                }
            }
        }
        min_steps
    }

    pub fn part2(&self) -> (Num, Pos) {
        let mut s = 0;
        let mut e = self.bytes.len();

        while s < e {
            let m = s + (e - s) / 2;
            let v = self.part1(m);
            if v == usize::MAX {
                e = m - 1;
            } else {
                s = m + 1;
            }
        }
        (s - 1, self.bytes[s - 1])
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc18 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc18/example").expect("couldn't read input - aoc18");;
        let context = Context::new(text, 7, 7);
        //println!("Context: {:?}", context);
        let part1 = context.part1(12);
        println!("Example1 Part1: {:?}", part1);
        println!("Example1 Part2: {:?}", context.part2());
    }

   #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc18/input").expect("couldn't read input - aoc18");
        let context = Context::new(text, 71, 71);
        let part1 = bench(|| context.part1(2911), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}