use crate::utils::{read_input, bench};
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::Hash;
use rayon::prelude::*;

type Height = i8;

// First is the current coord, second is the origin

#[derive(Debug)]
struct QueueObj {
    origin: (usize, usize),
    cur: (usize, usize),
    distance: usize,
}

impl QueueObj {
    pub fn new(origin: (usize, usize), cur: (usize, usize), distance: usize) -> QueueObj {
        QueueObj {
            origin: origin,
            cur: cur,
            distance: distance,
        }
    }
}

#[derive(Debug)]
struct Context {
    heights: Vec<Vec<Height>>,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        
        Context {
            heights: inp.into_iter()
                        .map(|s| s.chars().into_iter()
                                .map(|c| c.to_digit(10).unwrap() as Height)
                                .collect())
                        .collect()
        }
    }

    fn within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.heights.len() as isize && y >= 0 && y < self.heights[0].len() as isize
    }

    pub fn part1(&self) -> usize {
        let mut queue: VecDeque<QueueObj> = VecDeque::new();
        for i in 0..self.heights.len() {
            for j in 0..self.heights[0].len() {
                if self.heights[i][j] == 0 {
                    queue.push_back(QueueObj::new((i, j), (i, j), 0));
                }
            }
        }
        let mut visited: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        //println!("Trailheads: {:?}", queue);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            //println!("Current node: {:?}", node);
            let (iu, ju) = node.cur;
            let (i, j) = (iu as isize, ju as isize);
            let cur_height = self.heights[iu][ju];
            if cur_height == 9 {
                //println!("From {:?}", node.origin);
                visited.entry(node.origin)
                    .and_modify(|v| { v.insert((iu, ju)); })
                    .or_insert(HashSet::from_iter(vec![(iu, ju)]));
                continue;
            }
            let neighbours = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
            for (x, y) in neighbours {
                let (xu, yu) = (x as usize, y as usize);
                if self.within_bounds(x, y) && self.heights[xu][yu] == self.heights[iu][ju] + 1 {
                    queue.push_back(QueueObj::new(node.origin, (xu, yu), node.distance + 1));
                }
            }
        }
        visited.values().map(|v| v.len()).sum()
    }

    pub fn part2(&self) -> usize {
        let mut sum = 0;
        let mut queue: VecDeque<QueueObj> = VecDeque::new();
        for i in 0..self.heights.len() {
            for j in 0..self.heights[0].len() {
                if self.heights[i][j] == 0 {
                    queue.push_back(QueueObj::new((i, j), (i, j), 0));
                }
            }
        }
        let mut visited: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
        //println!("Trailheads: {:?}", queue);
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let (iu, ju) = node.cur;
            let (i, j) = (iu as isize, ju as isize);
            let cur_height = self.heights[iu][ju];
            if cur_height == 9 {
                sum += 1;
                continue;
            }
            let neighbours = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
            for (x, y) in neighbours {
                let (xu, yu) = (x as usize, y as usize);
                if self.within_bounds(x, y) && self.heights[xu][yu] == self.heights[iu][ju] + 1 {
                    queue.push_back(QueueObj::new(node.origin, (xu, yu), node.distance + 1));
                }
            }
        }
        //visited.values().map(|v| v.len()).sum()
        sum
    }

}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc10 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc10/example").expect("couldn't read input - aoc10");
        let context = Context::new(text);
        //println!("context: {:?}", context);
        let part1 = context.part1();
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc10/input").expect("couldn't read input - aoc10");
        let context = Context::new(text);
        let part1 = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}