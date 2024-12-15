use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::Hash;
use rayon::prelude::*;
use image::{DynamicImage, GrayImage, Pixel};

type Num = i32;

type Pos = (Num, Num);

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone, PartialEq)]
enum Type {
    Bot,
    Box,
    Block,
    Empty
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Bot => write!(f, "@"),
            Type::Box => write!(f, "O"),
            Type::Block => write!(f, "#"),
            Type::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct Context {
    bot: Pos,
    moves: Vec<Move>,
    matrix: Vec<Vec<Type>>
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let mut k = 0;
        let mut bot= (0, 0);
        let mut matrix: Vec<Vec<Type>> = vec![];
        let mut moves = vec![];
        loop {
            if inp[k].trim().is_empty() {
                k += 1;
                break;
            }
            let mut ve = vec![Type::Empty; inp[0].len()];
            for (j, c) in inp[k].chars().enumerate() {
                ve[j] = match c {
                    '#' => Type::Block,
                    '@' => {
                        bot = (k as Num, j as Num);
                        Type::Bot
                    },
                    'O' => Type::Box,
                    _ => Type::Empty,
                }
            }
            matrix.push(ve);
            k += 1;
        }
        while k < inp.len() {
            moves.extend(inp[k].chars().map(|c| match c {
                '>' => Move::Right,
                '^' => Move::Up,
                '<' => Move::Left,
                'v' => Move::Down,
                _ => panic!("Unknown move"),
            }));
            k += 1;
        }
        Context {
            bot: bot,
            moves: moves,
            matrix: matrix
        }
    }

    pub fn print_matrix(&self) {
        for i in 0..self.matrix.len() {
            println!("{:?}", self.matrix[i]);
        }
    }

    pub fn compute_gps(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[0].len() {
                if self.matrix[i][j] == Type::Box {
                    sum += 100 * i + j;
                }
            }
        }
        sum
    }

    fn move_bot(&mut self, i: usize) {
        //println!("Bot position: {:?}; Move: {:?}", self.bot, self.moves[i]);
        let add = match self.moves[i] {
            Move::Right => (0, 1),
            Move::Left => (0, -1),
            Move::Down => (1, 0),
            Move::Up => (-1, 0)
        };
        let nex_bot = (self.bot.0 + add.0, self.bot.1 + add.1);
        let nex_botu = (nex_bot.0 as usize, nex_bot.1 as usize);
        if self.matrix[nex_botu.0][nex_botu.1] == Type::Empty {
            self.matrix[nex_botu.0][nex_botu.1] = Type::Bot;
            self.matrix[self.bot.0 as usize][self.bot.1 as usize] = Type::Empty;
            self.bot = nex_bot;
            return;
        }
        if self.matrix[nex_botu.0][nex_botu.1] == Type::Block {
            return;
        }
        // If block ->
        // Keep moving in that direction until we hit one of the following
        // 1. Block -> Return
        // 2. Box -> Continue
        // 3. Empty -> Move all the blocks + bot by 1.
        let (mut i, mut j) = nex_bot;
        loop {
            let (iu, ju) = (i as usize, j as usize);
            if self.matrix[iu][ju] == Type::Block {
                return;
            }
            if self.matrix[iu][ju] == Type::Box {
                i += add.0;
                j += add.1;
                continue;
            }
            break;
        }
        //println!("Next Empty position at: ({}, {})", i, j);
        loop {
            let (ni, nj): (i32, i32) = (i - add.0, j - add.1);
            self.matrix[i as usize][j as usize] = self.matrix[ni as usize][nj as usize];
            if i == nex_bot.0 && j == nex_bot.1 {
                break;
            }
            i -= add.0;
            j -= add.1;
        }
        self.matrix[nex_botu.0][nex_botu.1] = Type::Bot;
        self.matrix[self.bot.0 as usize][self.bot.1 as usize] = Type::Empty;
        self.bot = nex_bot;
    }

    pub fn part1(&mut self) -> usize {
        //self.print_matrix();
        for i in 0..self.moves.len() {
            self.move_bot(i);
        }
        //self.print_matrix();
        self.compute_gps()
    }

}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc15 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc15/example1").expect("couldn't read input - aoc15");;
        let mut context = Context::new(text);
        //let part1 = context.part1(5, 11, 7);
        //println!("Example part1: {}", part1);
        let part1 = context.part1();
        println!("Example1 Part1: {:?}", part1);
        //println!("Example1 Part2: {:?}", context.part2());
    }

    #[test]
    fn example2() {
        let text: Vec<String> = read_input("src/aoc15/example2").expect("couldn't read input - aoc15");
        let mut context = Context::new(text);
        //println!("context: {:?}", context);
        let part1 = context.part1();
        println!("Example part1: {}", part1);
        //let (part1, area) = context.part1();
        //println!("Example1 Part1: {:?}", part1);
        //println!("Example1 Part2: {:?}", context.part2());
        //context.part2(1000, 11, 7);
    }

   #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc15/input").expect("couldn't read input - aoc15");
        let mut context = Context::new(text);
        let part1 = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        //println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
        //bench(|| context.part2(100000, 101, 103), Some("Part2"));
    }
}