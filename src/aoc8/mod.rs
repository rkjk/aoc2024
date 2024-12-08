use crate::utils::{read_input, bench};
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

type Location = (isize, isize);

#[derive(Debug)]
struct Context {
    antennas: HashMap<char, Vec<Location>>,
    rowsize: usize,
    colsize: usize,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let mut map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
        let rowsize = inp.len();
        let colsize = inp[0].len();
        for i in 0..inp.len() {
            for (j, c) in inp[i].chars().enumerate() {
                if c.is_ascii_uppercase() || c.is_ascii_lowercase() || c.is_numeric() {
                    map.entry(c)
                        .and_modify(|v| v.push((i as isize, j as isize)))
                        .or_insert(vec![(i as isize, j as isize)]);
                }
            }
        }
        Context {
            antennas: map,
            rowsize: rowsize,
            colsize: colsize
        }
    }

    fn within_grid(&self, l: Location) -> bool {
        l.0 >= 0 && l.0 < self.rowsize as isize && l.1 >= 0 && l.1 < self.colsize as isize
    }

    fn compute_antinodes(&self, loc1: &Location, loc2: &Location) -> Vec<Location> {
        let mut antinodes = vec![];
        let absr = (loc1.0 - loc2.0).abs();
        let absc = (loc1.1 - loc2.1).abs();
        let newx1 = loc1.0 - absr;
        let newy1 = match loc1.1 < loc2.1 {
            true => loc1.1 - absc,
            false => loc1.1 + absc
        };
        if self.within_grid((newx1, newy1)) {
            antinodes.push((newx1, newy1));
        }

        let newx2 = loc2.0 + absr;
        let newy2 = match loc1.1 < loc2.1 {
            true => loc2.1 + absc,
            false => loc2.1 - absc
        };
        if self.within_grid((newx2, newy2)) {
            antinodes.push((newx2, newy2));
        }
        antinodes
    }

    fn compute_resonant_antinodes(&self, loc1: &Location, loc2: &Location) -> Vec<Location> {
        let mut antinodes = vec![];
        let absr = (loc1.0 - loc2.0).abs();
        let absc = (loc1.1 - loc2.1).abs();

        let diffx1 = -absr;
        let diffy1 = match loc1.1 < loc2.1 {
            true => -absc,
            false => absc
        };
        let  (mut newx1, mut newy1) = *loc1;

        while self.within_grid((newx1 + diffx1, newy1 + diffy1)) {
            newx1 += diffx1;
            newy1 += diffy1;
            antinodes.push((newx1, newy1));
        }

        let  (mut newx2, mut newy2) = *loc2;
        let diffx2 = absr;
        let diffy2 = match loc1.1 < loc2.1 {
            true => absc,
            false => -absc
        };
        while self.within_grid((newx2 + diffx2, newy2 + diffy2)) {
            newx2 += diffx2;
            newy2 += diffy2;
            antinodes.push((newx2, newy2));
        }
        antinodes
    }

    fn helper(&self, key: &char, locs: &Vec<Location>, part2: bool) -> Vec<Location> {
        let mut antinodes = vec![];
        let mut new_locs = locs.clone();
        new_locs.sort_by_key(|l| l.0);
        for i in 0..new_locs.len() {
            for j in i+1..new_locs.len() {
                // if part2 {
                //     let v  = self.compute_resonant_antinodes(&locs[i], &locs[j]);
                //     println!("Antinodes for char: {}, pair: {:?} -> {:?}", key, (&locs[i],&locs[j]), v);
                // }
                match part2 {
                    false => antinodes.extend(self.compute_antinodes(&locs[i], &locs[j])),
                    true => antinodes.extend(self.compute_resonant_antinodes(&locs[i], &locs[j])),
                };
            }
        }
        antinodes
    }

    pub fn part1(&self) -> usize {
        let antinodes: Vec<Location> = self.antennas.iter()
            .map(|(k, v)| self.helper(k, v, false))
            .flatten()
            .collect();
        let set: HashSet<Location> = antinodes.into_iter().collect();
        //println!("set: {:?}", set);
        set.len()
    }

    pub fn part2(&self) -> usize {
        let mut antinodes: Vec<Location> = self.antennas.iter()
            .map(|(k, v)| self.helper(k, v, true))
            .flatten()
            .collect();
        antinodes.extend(self.antennas.values().flatten());
        let set: HashSet<Location> = antinodes.into_iter().collect();
        //println!("set part2: {:?}", set);
        set.len()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc8 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc8/example").expect("couldn't read input - aoc8");
        let context = Context::new(text);
        //println!("context: {:?}", context);
        let part1 = context.part1();
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc8/input").expect("couldn't read input - aoc8");
        let context = Context::new(text);
        let part1 = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
    }
}