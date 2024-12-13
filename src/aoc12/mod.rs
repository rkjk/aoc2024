use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use rayon::prelude::*;

#[derive(Debug)]
struct Context {
    plots: Vec<Vec<u8>>,
    connected_components: Vec<Vec<usize>>,
    r: usize,
    c: usize,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let (r, c) = (inp.len(), inp[0].len());
        Context {
            plots: inp.into_iter().map(|v| v.into_bytes()).collect(),
            connected_components: vec![vec![0; c]; r],
            r: r,
            c: c
        }
    }

    fn check_out_of_bounds(&self, ni: isize, nj: isize) -> bool {
        ni < 0 || ni >= self.r as isize || nj < 0 || nj >= self.c as isize
    }

    fn check_out_of_boundsu(&self, i: usize, j: usize) -> bool {
        self.check_out_of_bounds(i as isize, j as isize)
    }

    fn neighbors(i: usize, j: usize) -> [(isize, isize); 4] {
        let (ii, ji) = (i as isize, j as isize);
        [(ii + 1, ji), (ii - 1, ji), (ii, ji + 1), (ii, ji - 1)]
    }

    fn visit_node(&mut self, i:usize, j: usize, visited: &mut Vec<Vec<bool>>, component_number: usize) {
        if visited[i][j] {
            return;
        }
        visited[i][j] = true;
        self.connected_components[i][j] = component_number;
        for (ni, nj) in Context::neighbors(i, j) {
            if  self.check_out_of_bounds(ni, nj) || self.plots[ni as usize][nj as usize] != self.plots[i][j] {
                continue;
            }
            self.visit_node(ni as usize, nj as usize, visited, component_number);
        }
    }

    pub fn compute_connected_components(&mut self) -> usize {
        let mut component_counter = 0;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.c]; self.r];
        for i in 0..self.r {
            for j in 0..self.c {
                if visited[i][j] {
                    continue;
                }
                component_counter += 1;
                self.visit_node(i, j, &mut visited, component_counter);

            }
        }
        component_counter
    }

    fn compute_perimeter(&self, i: usize, j: usize) -> usize {
        let mut sum = 0;
        for (ni, nj) in Context::neighbors(i, j) {
            if self.check_out_of_bounds(ni, nj) {
                sum += 1;
            } else if self.connected_components[i][j] != self.connected_components[ni as usize][nj as usize] {
                sum += 1;
            }
        }
        sum
    }

    fn check_same_connected_component(&self, i: isize, j: isize, component_id: usize) -> bool {
        self.check_same_connected_componentu(i as usize, j as usize, component_id)
    }

    fn check_same_connected_componentu(&self, i: usize, j: usize, component_id: usize) -> bool {
        self.connected_components[i][j] == component_id
    }

    fn twodiff(&self, i1: isize, j1: isize, i2: isize, j2: isize, component_id: usize) -> bool {
        // Corner type1
        //    C
        //  C X
        let one: bool = self.check_out_of_bounds(i1, j1) || !self.check_same_connected_component(i1, j1, component_id);
        let two: bool = self.check_out_of_bounds(i2, j2) || !self.check_same_connected_component(i2, j2, component_id);
        one && two
    }

    fn threesame(&self, i1: isize, j1: isize, i2: isize, j2: isize, i3: isize, j3: isize, component_id: usize) -> bool {
        let one = !self.check_out_of_bounds(i1, j1) && self.check_same_connected_component(i1, j1, component_id);
        let two: bool = !self.check_out_of_bounds(i2, j2) && !self.check_same_connected_component(i2, j2, component_id);
        let three: bool = self.check_out_of_bounds(i3, j3) || self.check_same_connected_component(i3, j3, component_id);
        one && two && three
    }

    fn compute_sides(&self, i: usize, j: usize) -> usize {
        let mut corners = 0;
        let component_id = self.connected_components[i][j];
        let (ii, ji) = (i as isize, j as isize);
        // Top Left
        let (i1, j1) = (ii, ji - 1);
        let (i2, j2) = (ii - 1, ji);
        let (i3, j3) = (ii - 1, ji - 1);
        if self.twodiff(i1, j1, i2, j2, component_id) || self.threesame(i1, j1, i2, j2, i3, j3, component_id) {
            corners += 1;
        }
        // Top Right
        let (i1, j1) = (ii, ji + 1);
        let (i2, j2) = (ii - 1, ji);
        let (i3, j3) = (ii - 1, ji + 1);
        if self.twodiff(i1, j1, i2, j2, component_id) || self.threesame(i1, j1, i2, j2, i3, j3, component_id) {
            corners += 1;
        }
        // Bottom Left
        let (i1, j1) = (ii, ji - 1);
        let (i2, j2) = (ii + 1, ji);
        let (i3, j3) = (ii + 1, ji - 1);
        if self.twodiff(i1, j1, i2, j2, component_id) || self.threesame(i1, j1, i2, j2, i3, j3, component_id) {
            corners += 1;
        }
        // Bottom Right
        let (i1, j1) = (ii, ji + 1);
        let (i2, j2) = (ii + 1, ji);
        let (i3, j3) = (ii + 1, ji + 1);
        if self.twodiff(i1, j1, i2, j2, component_id) || self.threesame(i1, j1, i2, j2, i3, j3, component_id) {
            corners += 1;
        }
        //println!("Corners: {:?}", corners);
        corners
    }

    pub fn part1(&mut self) -> (usize, Vec<usize>) {
        let num_components = self.compute_connected_components();
        let (mut area, mut perimeter) = (vec![0; num_components + 1], vec![0; num_components + 1]);
        for i in 0..self.r {
            for j in 0..self.c {
                let component_id = self.connected_components[i][j];
                area[component_id] += 1;
                perimeter[component_id] += self.compute_perimeter(i, j);
            }
        }
        (area.iter().zip(perimeter).map(|(a, p)| a*p).sum(), area)
    }

    pub fn part2(&self, num_components: usize, area: Vec<usize>) -> usize {
        //let mut sides = vec![HashSet::new(); num_components + 1];
        let mut sides = vec![0; num_components];
        //println!("Num components: {}", num_components);
        for i in 0..self.r {
            for j in 0..self.c {
                let component_id = self.connected_components[i][j];
                sides[component_id] += self.compute_sides(i, j);
            }
        }
        //println!("components: {:?}", self.connected_components);
        //println!("Sides: {:?}", sides);
        area.iter().zip(sides).map(|(a, p)| a*p).sum()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc12 {
    use core::num;

    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc12/example1").expect("couldn't read input - aoc12");
        let mut context = Context::new(text);
        //let num_components = context.compute_connected_components();
        //println!("Num components: {}", num_components);
        //println!("Component Grid: {:?}", context.connected_components);
        //println!("split test: {:?}", context.split_number(&2000));
        //println!("context: {:?}", context);
        let (part1, area) = context.part1();
        println!("Example1 Part1: {:?}", part1);
        println!("Example1 Part2: {:?}", context.part2(area.len(), area));
    }

   #[test]
    fn example2() {
        let text: Vec<String> = read_input("src/aoc12/example2").expect("couldn't read input - aoc12");
        let mut context = Context::new(text);
        //let num_components = context.compute_connected_components();
        //println!("Num components: {}", num_components);
        //println!("Component Grid: {:?}", context.connected_components);
        //println!("split test: {:?}", context.split_number(&2000));
        //println!("context: {:?}", context);
        let (part1, area) = context.part1();
        println!("Example2 Part1: {:?}", part1);
        println!("Example2 Part2: {:?}", context.part2(part1, area));
    }

    #[test]
    fn example3() {
        let text: Vec<String> = read_input("src/aoc12/example3").expect("couldn't read input - aoc12");
        let mut context = Context::new(text);
        //let num_components = context.compute_connected_components();
        //println!("Num components: {}", num_components);
        //println!("Component Grid: {:?}", context.connected_components);
        //println!("split test: {:?}", context.split_number(&2000));
        //println!("context: {:?}", context);
        let (part1, area) = context.part1();
        println!("Example3 Part1: {:?}", part1);
        println!("Example3 Part2: {:?}", context.part2(part1, area));
    }

    #[test]
    fn example4() {
        let text: Vec<String> = read_input("src/aoc12/example4").expect("couldn't read input - aoc12");
        let mut context = Context::new(text);
        //let num_components = context.compute_connected_components();
        //println!("Num components: {}", num_components);
        //println!("Component Grid: {:?}", context.connected_components);
        //println!("split test: {:?}", context.split_number(&2000));
        //println!("context: {:?}", context);
        let (part1, area) = context.part1();
        println!("Example4 Part1: {:?}", part1);
        println!("Example4 Part2: {:?}", context.part2(part1, area));
    }

    #[test]
    fn example5() {
        let text: Vec<String> = read_input("src/aoc12/example5").expect("couldn't read input - aoc12");
        let mut context = Context::new(text);
        //let num_components = context.compute_connected_components();
        //println!("Num components: {}", num_components);
        //println!("Component Grid: {:?}", context.connected_components);
        //println!("split test: {:?}", context.split_number(&2000));
        //println!("context: {:?}", context);
        let (part1, area) = context.part1();
        println!("Example5 Part1: {:?}", part1);
        println!("Example5 Part2: {:?}", context.part2(part1, area));
    }

   #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc12/input").expect("couldn't read input - aoc12");
        let mut context = Context::new(text);
        let num_components = context.compute_connected_components();
        //println!("Num components: {}", num_components);
        //println!("Component Grid: {:?}", context.connected_components);
        let (part1, area) = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(part1, area), Some("part2")));
    }
}