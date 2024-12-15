use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::Hash;
use rayon::prelude::*;
use image::{DynamicImage, GrayImage, Pixel};

type Num = i32;

type Pos = (Num, Num);

#[derive(Debug, PartialEq)]
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
    BoxLeft,
    BoxRight,
    Empty
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Bot => write!(f, "@"),
            Type::Box => write!(f, "O"),
            Type::Block => write!(f, "#"),
            Type::Empty => write!(f, "."),
            Type::BoxLeft => write!(f, "["),
            Type::BoxRight => write!(f, "]")
        }
    }
}

#[derive(Debug)]
struct Context {
    bot: Pos,
    moves: Vec<Move>,
    matrix: Vec<Vec<Type>>,
    bot2: Pos,
    matrix2: Vec<Vec<Type>>,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let mut k = 0;
        let mut bot= (0, 0);
        let mut bot2= (0, 0);
        let mut matrix: Vec<Vec<Type>> = vec![];
        let mut matrix2: Vec<Vec<Type>> = vec![];
        let mut moves = vec![];
        loop {
            if inp[k].trim().is_empty() {
                k += 1;
                break;
            }
            let mut ve = vec![Type::Empty; inp[0].len()];
            let mut ve2 = vec![];
            for (j, c) in inp[k].chars().enumerate() {
                ve[j] = match c {
                    '#' => Type::Block,
                    '@' => {
                        bot = (k as Num, j as Num);
                        Type::Bot
                    },
                    'O' => Type::Box,
                    _ => Type::Empty,
                };
                match c {
                    '#' => ve2.extend(vec![Type::Block, Type::Block]),
                    '@' => {
                        bot2 = (k as Num, ve2.len() as Num);
                        ve2.extend(vec![Type::Bot, Type::Empty]);
                    },
                    'O' => ve2.extend(vec![Type::BoxLeft, Type::BoxRight]),
                    _ => ve2.extend(vec![Type::Empty, Type::Empty]),
                }
            }
            matrix.push(ve);
            matrix2.push(ve2);
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
            matrix: matrix,
            bot2: bot2,
            matrix2: matrix2
        }
    }

    pub fn print_matrix(&self) {
        for i in 0..self.matrix.len() {
            println!("{:?}", self.matrix[i]);
        }
    }

    pub fn print_matrix2(&self) {
        for i in 0..self.matrix2.len() {
            println!("{:?}", self.matrix2[i]);
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

    pub fn compute_gps2(&self) -> usize {
        let mut sum = 0;
        for i in 0..self.matrix2.len() {
            for j in 0..self.matrix2[0].len() {
                if self.matrix2[i][j] == Type::BoxLeft {
                    sum += 100 * i + j;
                }
            }
        }
        sum
    }

    fn move_bot(bot: &mut Pos, matrix: &mut Vec<Vec<Type>>, moves: &Vec<Move>, i: usize) {
        //println!("Bot position: {:?}; Move: {:?}", bot, moves[i]);
        let add = match moves[i] {
            Move::Right => (0, 1),
            Move::Left => (0, -1),
            Move::Down => (1, 0),
            Move::Up => (-1, 0)
        };
        let nex_bot = (bot.0 + add.0, bot.1 + add.1);
        let nex_botu = (nex_bot.0 as usize, nex_bot.1 as usize);
        if matrix[nex_botu.0][nex_botu.1] == Type::Empty {
            matrix[nex_botu.0][nex_botu.1] = Type::Bot;
            matrix[bot.0 as usize][bot.1 as usize] = Type::Empty;
            *bot = nex_bot;
            return;
        }
        if matrix[nex_botu.0][nex_botu.1] == Type::Block {
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
            if matrix[iu][ju] == Type::Block {
                return;
            }
            if matrix[iu][ju] == Type::Box || matrix[iu][ju] == Type::BoxLeft || matrix[iu][ju] == Type::BoxRight {
                i += add.0;
                j += add.1;
                continue;
            }
            break;
        }
        //println!("Next Empty position at: ({}, {})", i, j);
        loop {
            let (ni, nj): (i32, i32) = (i - add.0, j - add.1);
            matrix[i as usize][j as usize] = matrix[ni as usize][nj as usize];
            if i == nex_bot.0 && j == nex_bot.1 {
                break;
            }
            i -= add.0;
            j -= add.1;
        }
        matrix[nex_botu.0][nex_botu.1] = Type::Bot;
        matrix[bot.0 as usize][bot.1 as usize] = Type::Empty;
        *bot = nex_bot;
    }

    pub fn part1(&mut self) -> usize {
        //self.print_matrix();
        for i in 0..self.moves.len() {
            //println!("");
            Context::move_bot(&mut self.bot, &mut self.matrix, &self.moves, i);
        }
        //self.print_matrix();
        self.compute_gps()
    }

    fn check_column(cur_pos: Pos, m: &Move, matrix: &Vec<Vec<Type>>) -> bool {
        let add = match m {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            _ => panic!("Cannot do this for lateral moves")
        };
        let nex_pos = (cur_pos.0 + add.0, cur_pos.1 + add.1);
        let nex_posu = (nex_pos.0 as usize, nex_pos.1 as usize);
        let cur_posu = (cur_pos.0 as usize, cur_pos.1 as usize);
        if matrix[nex_posu.0][nex_posu.1] == Type::Block {
            return false;
        }
        if matrix[nex_posu.0][nex_posu.1] == Type::Empty {
            return true;
        }
        if matrix[nex_posu.0][nex_posu.1] == Type::BoxLeft {
            return Context::check_column((nex_pos.0, nex_pos.1), m, matrix) &&
                Context::check_column((nex_pos.0, nex_pos.1 + 1), m, matrix);
        }
        if matrix[nex_posu.0][nex_posu.1] == Type::BoxRight {
            return Context::check_column((nex_pos.0, nex_pos.1), m, matrix) &&
            Context::check_column((nex_pos.0, nex_pos.1 - 1), m, matrix);
        }
        panic!("Unknown type for check_column: {:?}", matrix[nex_posu.0][nex_posu.1]);
    }

    fn move_column(cur_pos: Pos, m: &Move, matrix: &mut Vec<Vec<Type>>) -> Pos {
        let add = match m {
            Move::Up => (-1, 0),
            Move::Down => (1, 0),
            _ => panic!("Cannot do this for lateral moves")
        };
        let nex_pos = (cur_pos.0 + add.0, cur_pos.1 + add.1);
        let nex_posu = (nex_pos.0 as usize, nex_pos.1 as usize);
        let cur_posu = (cur_pos.0 as usize, cur_pos.1 as usize);
        if matrix[nex_posu.0][nex_posu.1] == Type::Block {
            panic!("Cannot move {:?} with move {:?}", cur_pos, m);
        }
        if matrix[nex_posu.0][nex_posu.1] == Type::Empty {
            matrix[nex_posu.0][nex_posu.1] = matrix[cur_posu.0][cur_posu.1];
            matrix[cur_posu.0][cur_posu.1] = Type::Empty;
            //println!("Cur_pos: {:?}, Value: {:?}, new_pos: {:?}", cur_pos, matrix[cur_posu.0][cur_posu.1], nex_pos);
            return nex_pos;
        }
        if matrix[nex_posu.0][nex_posu.1] == Type::BoxLeft {
            Context::move_column((nex_pos.0, nex_pos.1), m, matrix);
            Context::move_column((nex_pos.0, nex_pos.1 + 1), m, matrix);
            matrix[nex_posu.0][nex_posu.1] = matrix[cur_posu.0][cur_posu.1];
            matrix[cur_posu.0][cur_posu.1] = Type::Empty;
            //println!("Cur_pos: {:?}, Value: {:?}, new_pos: {:?}", cur_pos, matrix[cur_posu.0][cur_posu.1], nex_pos);
            return nex_pos;
        }
        if matrix[nex_posu.0][nex_posu.1] == Type::BoxRight {
            Context::move_column((nex_pos.0, nex_pos.1), m, matrix);
            Context::move_column((nex_pos.0, nex_pos.1 - 1), m, matrix);
            matrix[nex_posu.0][nex_posu.1] = matrix[cur_posu.0][cur_posu.1];
            matrix[cur_posu.0][cur_posu.1] = Type::Empty;
            //println!("Cur_pos: {:?}, Value: {:?}, new_pos: {:?}", cur_pos, matrix[cur_posu.0][cur_posu.1], nex_pos);
            return nex_pos;
        }
        panic!("Unknown type for check_column: {:?}", matrix[nex_posu.0][nex_posu.1]);
    }

    fn move_bot_2(bot: &mut Pos, matrix: &mut Vec<Vec<Type>>, moves: &Vec<Move>, i: usize) {
        if moves[i] == Move::Left || moves[i] == Move::Right {
            Context::move_bot(bot, matrix, moves, i);
            return;
        }
        //println!("Vertical move");
        if Context::check_column(*bot, &moves[i], &matrix) {
            let nex_pos = Context::move_column(*bot, &moves[i], matrix);
            matrix[bot.0 as usize][bot.1 as usize] = Type::Empty;
            *bot = nex_pos;
        }
    }

    pub fn part2(&mut self) -> usize {
        //self.print_matrix2();
         for i in 0..self.moves.len() {
            //println!("");
            Context::move_bot_2(&mut self.bot2, &mut self.matrix2, &self.moves, i);
            //self.print_matrix2();
         }
        //self.print_matrix2();  
        self.compute_gps2()
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
        println!("Example1 Part2: {:?}", context.part2());
    }

    #[test]
    fn example2() {
        let text: Vec<String> = read_input("src/aoc15/example2").expect("couldn't read input - aoc15");
        let mut context = Context::new(text);
        //println!("context: {:?}", context);
        let part1 = context.part1();
        println!("Example2 part1: {}", part1);
        //let (part1, area) = context.part1();
        //println!("Example1 Part1: {:?}", part1);
        println!("Example2 Part2: {:?}", context.part2());
    }

    #[test]
    fn example3() {
        let text: Vec<String> = read_input("src/aoc15/example3").expect("couldn't read input - aoc15");;
        let mut context = Context::new(text);
        let part1 = context.part1();
        println!("Example3 part1: {}", part1);
        //let part1 = context.part1();
        //println!("Example3 Part1: {:?}", part1);
        println!("Example3 Part2: {:?}", context.part2());
    }

   #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc15/input").expect("couldn't read input - aoc15");
        let mut context = Context::new(text);
        let part1 = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}