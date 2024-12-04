use crate::utils::{read_input, bench};
use regex::Regex;
use std::cmp::PartialEq;

#[derive(Debug)]
struct Context {
    letters: Vec<Vec<u8>>,
}

const vals: &[u8] = &['X' as u8, 'M' as u8, 'A' as u8, 'S' as u8];
const permutations: &[(isize, isize)] = &[(0,1), (0,-1), (1,0), (-1, 0), (-1,1), (-1, -1), (1, 1), (1, -1)];

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        Context {
            letters: inp.into_iter().map(|s| Vec::from(s.as_bytes())).collect()
        }
    }

    fn count_xmas(&self, i: isize, j: isize) -> u32 {
        let row_len = self.letters.len() as isize;
        let col_len = self.letters[0].len() as isize;
        let mut sum = 8;
        for (di, dj) in permutations {
            for k in 0..4 {
                let ni = i + k * di;
                let nj = j + k * dj;
                if ni < 0 || ni >= row_len || nj < 0 || nj >= col_len || self.letters[ni as usize][nj as usize] != vals[k as usize] {
                    sum -= 1;
                    break;
                }
            }
        }
        sum
    }

    pub fn part1(&self) -> u32 {
        let mut sum = 0;
        for i in 0..self.letters.len() {
            for j in 0..self.letters[0].len() {
                //let c = self.count_xmas(i as isize, j as isize);
                //println!("i: {}, j: {}, count: {}", i, j, c);
                sum += self.count_xmas(i as isize, j as isize);
            }
        }
        sum
    }

    pub fn part2(&self) -> u32 {
        let A = 'A' as u8;
        let cp1 = (vals[1], vals[3]);
        let cp2 = (vals[3], vals[1]);
        let row_len = self.letters.len() as isize;
        let col_len = self.letters[0].len() as isize;
    
        let mut sum = 0;
        for i in 0..row_len as usize {
            for j in 0..col_len as usize {
                let (ii, ji) = (i as isize, j as isize);
                if self.letters[i][j] != A {
                    continue;
                }
                if ii - 1 < 0 || ji - 1 < 0 || ii + 1 >= row_len || ji + 1 >= col_len {
                    continue;
                }
                let pair1 = (self.letters[i + 1][j - 1], self.letters[i - 1][j + 1]);
                let pair2 = (self.letters[i - 1][j - 1], self.letters[i + 1][j + 1]);
                if (pair1 == cp1 || pair1 == cp2) && (pair2 == cp1 || pair2 == cp2) {
                    sum += 1;
                }
            }
        }
        sum
    }
}



#[cfg(test)]
mod aoc4 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc4/example").expect("couldn't read input - aoc4");
        let context = Context::new(text);
        println!("Part1: {:?}", context.part1());
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc4/input").expect("couldn't read input - aoc4");
        let context = Context::new(text);
        println!("Part1: {:?}", bench(|| context.part1(), Some("part1")));
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}