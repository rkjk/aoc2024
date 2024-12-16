use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::Hash;
use std::ops::Index;
use rayon::prelude::*;
use image::{DynamicImage, GrayImage, Pixel};

type Num = i32;

type Pos = (Num, Num);

#[derive(Copy, Clone, PartialEq)]
enum Type {
    Block,
    Empty
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Block => write!(f, "#"),
            Type::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct State {
    pos: Pos,
    m: Pos,
    c: usize,
    visited: Vec<Pos>
}

#[derive(Debug)]
struct Context {
    start: Pos,
    end: Pos,
    matrix: Vec<Vec<Type>>,
    moves: Vec<Pos>
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let mut start = (0, 0);
        let mut end= (0, 0);
        let mut matrix: Vec<Vec<Type>> = vec![];
        for k in 0..inp.len() {
            let mut ve = vec![Type::Empty; inp[0].len()];
            for (j, c) in inp[k].chars().enumerate() {
                ve[j] = match c {
                    '#' => Type::Block,
                    'S' => {
                        start = (k as Num, j as Num);
                        Type::Empty
                    },
                    'E' => {
                        end = (k as Num, j as Num);
                        Type::Empty
                    },
                    _ => Type::Empty,
                };
            }
            matrix.push(ve);
        }
        Context {
            start: start,
            end: end,
            matrix: matrix,
            moves: vec![(0, 1), (1, 0), (0, -1), (-1, 0)], // ESWN
        }
    }

    fn get_move_clockwise(&self, cur_move: &Pos) -> Pos {
        if let Some(index) = self.moves.iter().position(|&x| x.0 == cur_move.0 && x.1 == cur_move.1) {
            return self.moves[(index + 1) % 4];
        } else {
            panic!("Move not found");
        }    
    }

    fn get_move_anticlockwise(&self, cur_move: &Pos) -> Pos {
        if let Some(index) = self.moves.iter().position(|&x| x.0 == cur_move.0 && x.1 == cur_move.1) {
            if index == 0 {
                return self.moves[3];
            }
            return self.moves[index - 1];
        } else {
            panic!("Move not found");
        }    
    }

    pub fn print_matrix(&self) {
        //println!("start: ({}, {})", self.start.0, self.start.1);
        //println!("end: ({}, {})", self.end.0, self.end.1);
        for i in 0..self.matrix.len() {
            println!("{:?}", self.matrix[i]);
        }
    }

    pub fn part1(&self) -> (usize, Vec<Vec<usize>>) {
        let mut cost = vec![vec![usize::MAX; self.matrix[0].len()]; self.matrix.len()];
        let mut q = VecDeque::new();
        q.push_back((self.start, self.moves[0], 0));
        while !q.is_empty() {
            let (pos, m, c) = q.pop_front().unwrap();
            let posu = (pos.0 as usize, pos.1 as usize);
            if cost[posu.0][posu.1] > c {
                cost[posu.0][posu.1] = c;
            } else {
                continue;
            }
            if pos.0 == self.end.0 && pos.1 == self.end.1 {
                continue;
            }
            let n1 = (pos.0 + m.0, pos.1 + m.1);
            if self.not_block(&n1) {
                q.push_back((n1, m, c + 1));
            }
            let m2 = self.get_move_clockwise(&m);
            let n2 = (pos.0 + m2.0, pos.1 + m2.1);
            if self.not_block(&n2) {
                q.push_back((n2, m2, c + 1001));
            }
            let m3 = self.get_move_anticlockwise(&m);
            let n3 = (pos.0 + m3.0, pos.1 + m3.1);
            if self.not_block(&n3) {
                q.push_back((n3, m3, c + 1001));
            }
        }
        (cost[self.end.0 as usize][self.end.1 as usize], cost)
    }

    fn not_block(&self, x: &Pos) -> bool { 
        self.matrix[x.0 as usize][x.1 as usize] == Type::Empty 
    }

    fn dfs(&self, cost: &Vec<Vec<usize>>, pos: Pos, m: Pos, c: usize, visited: &mut Vec<Vec<bool>>) -> HashSet<(usize, usize)> {
        if pos.0 == self.end.0 && pos.1 == self.end.1 {
            if c != cost[pos.0 as usize][pos.1 as usize] {
                return HashSet::new();
            }
            let mut set = HashSet::new();
            for i in 0..visited.len() {
                for j in 0..visited[0].len() {
                    if visited[i][j] {
                        set.insert((i, j));
                    }
                }
            }
            println!("Length: {}", set.len() + 1);
            return set;
        }
        let posu = (pos.0 as usize, pos.1 as usize);
        if cost[posu.0][posu.1] > c || visited[posu.0][posu.1] || c >= cost[self.end.0 as usize][self.end.1 as usize] {
            return HashSet::new();
        }
        println!("Current node: {:?}", pos);
        visited[posu.0][posu.1] = true;
        let n1 = (pos.0 + m.0, pos.1 + m.1);
        let mut set = HashSet::new();
        if self.not_block(&n1) {
            set.extend(self.dfs(cost, n1, m, c + 1, visited).into_iter());
        }
        let m2 = self.get_move_clockwise(&m);
        let n2 = (pos.0 + m2.0, pos.1 + m2.1);
        if self.not_block(&n2) {
            set.extend(self.dfs(cost, n2, m2, c + 1001, visited).into_iter());
       
        }
        let m3 = self.get_move_anticlockwise(&m);
        let n3 = (pos.0 + m3.0, pos.1 + m3.1);
        if self.not_block(&n3) {
            set.extend(self.dfs(cost, n3, m3, c + 1001, visited).into_iter());
       
        }
        visited[posu.0][posu.1] = false;
        set
    }

    pub fn part2(&self, cost: &Vec<Vec<usize>>) -> usize {
        //let mut visited = vec![vec![false; self.matrix[0].len()]; self.matrix.len()];
        // let set = self.dfs(cost, self.start, (0, 1), 0, &mut visited);
        // set.len() + 1
        let mut q: VecDeque<((i32, i32), (i32, i32), usize, Vec<Pos>)> = VecDeque::new();
        q.push_back((self.start, self.moves[0], 0 as usize, vec![]));
        let mut set = HashSet::new();
        while !q.is_empty() {
            let (pos, m, c, mut visited) = q.pop_front().unwrap();
            let posu = (pos.0 as usize, pos.1 as usize);
            if pos.0 == self.end.0 && pos.1 == self.end.1 {
                for v in visited {
                    set.insert(v);
                }
                continue;
            }
            if c >= cost[posu.0][posu.1] + 7000 || c > cost[self.end.0 as usize][self.end.1 as usize] {
                continue;
            }
            let n1 = (pos.0 + m.0, pos.1 + m.1);
            visited.push(pos);
            if self.not_block(&n1) {
                q.push_back((n1, m, c + 1, visited.clone()));
            }
            let m2 = self.get_move_clockwise(&m);
            let n2 = (pos.0 + m2.0, pos.1 + m2.1);
            if self.not_block(&n2) {
                q.push_back((n2, m2, c + 1001, visited.clone()));
            }
            let m3 = self.get_move_anticlockwise(&m);
            let n3 = (pos.0 + m3.0, pos.1 + m3.1);
            if self.not_block(&n3) {
                q.push_back((n3, m3, c + 1001, visited.clone()));
            }
        }
        set.len() + 1
    }

}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc16 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc16/example1").expect("couldn't read input - aoc16");;
        let context = Context::new(text);
        context.print_matrix();
        let (part1, cost) = context.part1();
        println!("Example1 Part1: {:?}", part1);
        println!("Example1 Part2: {:?}", context.part2(&cost));
    }

    #[test]
    fn example2() {
        let text: Vec<String> = read_input("src/aoc16/example2").expect("couldn't read input - aoc16");
        let mut context = Context::new(text);
        //println!("context: {:?}", context);
        let (part1, cost) = context.part1();
        println!("Example2 part1: {}", part1);
        //let (part1, area) = context.part1();
        //println!("Example1 Part1: {:?}", part1);
        println!("Example2 Part2: {:?}", context.part2(&cost));
    }

   #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc16/input").expect("couldn't read input - aoc16");
        let mut context = Context::new(text);
        let (part1, cost) = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(&cost), Some("part2")));
    }
}