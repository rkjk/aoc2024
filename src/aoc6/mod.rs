use crate::utils::{read_input, bench};
use std::fmt::{Debug, Formatter, Result};
use std::cmp::PartialEq;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Brick
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Brick => write!(f, "#"),
        }
    }
}

struct Context {
    matrix: Vec<Vec<Tile>>,
    init_pos: (usize, usize)
}

impl Debug for Context {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Print each row of the matrix on a new line
        writeln!(f)?;
        for row in &self.matrix {
            for tile in row {
                write!(f, "{:?}", tile)?;
            }
            writeln!(f)?;
        }

        // Leave two blank lines
        writeln!(f)?;
        writeln!(f)?;

        // Print the init_pos
        write!(f, "init_pos: {:?}", self.init_pos)
    }
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let mut matrix = vec![];
        let mut init_pos: (usize, usize) = (usize::MAX, usize::MAX);
        for i in 0..inp.len() {
            let mut v = vec![];
            for (j, c) in inp[i].chars().enumerate() {
                match c {
                    '.' => v.push(Tile::Empty),
                    '#' => v.push(Tile::Brick),
                    '^' => {
                        v.push(Tile::Empty);
                        init_pos = (i, j);
                    },
                    _ => panic!("Unknown character"),
                };
            }
            matrix.push(v.into());
        }
        Context {
            matrix: matrix,
            init_pos: init_pos
        }
    }

    fn out_of_bounds(&self, x: &isize, y: &isize) -> bool{
        if *x < 0 || *x >= self.matrix.len() as isize || *y < 0 || *y >= self.matrix[0].len() as isize {
            return true;
        }
        false
    }

    pub fn part1(&self) -> (usize, Vec<(usize, usize)>) {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.matrix[0].len()]; self.matrix.len()];
        visited[self.init_pos.0][self.init_pos.1] = true;
        let (mut x, mut y) = (self.init_pos.0 as isize, self.init_pos.1 as isize);
        let (mut del_x, mut del_y): (isize, isize) = (-1, 0);
        loop {
            let (mut new_x, mut new_y) = (x + del_x, y + del_y);
            if self.out_of_bounds(&new_x, &new_y) {
                break;
            }
            if self.matrix[new_x as usize][new_y as usize] == Tile::Brick {
                let tmp = del_x;
                del_x = del_y;
                del_y = -1 * tmp;
                new_x = x + del_x;
                new_y = y + del_y;
            }
            x = new_x;
            y = new_y;
            visited[x as usize][y as usize] = true;
        }
        let mut guard_path = vec![];
        for i in 0..visited.len() {
            for j in 0..visited[0].len() {
                if visited[i][j] && (i, j) != self.init_pos {
                    guard_path.push((i, j));
                }
            }
        }
        (visited.iter().flatten().filter(|&x| *x).count(), guard_path)
    }

    fn helper(&self, obstruction: (isize, isize)) -> bool {
        let mut visited: Vec<Vec<(isize, isize)>>  = vec![vec![(isize::MAX, isize::MAX); self.matrix[0].len()]; self.matrix.len()];
        let (mut x, mut y) = (self.init_pos.0 as isize, self.init_pos.1 as isize);
        let (mut del_x, mut del_y): (isize, isize) = (-1, 0);
        visited[self.init_pos.0][self.init_pos.1] = (del_x, del_y);
        loop {
            let (mut new_x, mut new_y) = (x + del_x, y + del_y);
            if self.out_of_bounds(&new_x, &new_y) {
                return false;
            }   
            while self.matrix[new_x as usize][new_y as usize] == Tile::Brick || obstruction == (new_x, new_y) {
                let tmp = del_x;
                del_x = del_y;
                del_y = -1 * tmp;
                new_x = x + del_x;
                new_y = y + del_y;
            }
            if (del_x, del_y) == visited[new_x as usize][new_y as usize] {
                return true;
            }
            x = new_x;
            y = new_y;
            visited[x as usize][y as usize] = (del_x, del_y);
        }
    }

    pub fn part2(&self, guard_path: &Vec<(usize, usize)>) -> usize {
        let mut sum = 0;
        for (i, j) in guard_path {
            match self.helper((*i as isize, *j as isize)) {
                true => sum += 1,
                false => (),
            };
        }
        sum
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc6 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc6/example").expect("couldn't read input - aoc6");
        let context = Context::new(text);
        let (part1, path) = context.part1();
        println!("Guard path length: {}", path.len());
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", context.part2(&path));
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc6/input").expect("couldn't read input - aoc6");
        let context = Context::new(text);
        let (part1, path) = bench(|| context.part1(), Some("part1"));
        println!("Guard path length: {}", path.len());
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(&path), Some("part2")));
    }
}