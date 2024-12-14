use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::Hash;
use rayon::prelude::*;
use image::{DynamicImage, GrayImage, Pixel};

type Num = i32;

type Pos = (Num, Num);

struct Bot {
    position: Pos,
    velocity: Pos
}

impl Debug for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Bot {{ p=({}, {}), v=({}, {}) }}", 
               self.position.0, self.position.1, 
               self.velocity.0, self.velocity.1)
    }
}

impl Bot {
    // Constructor method
    pub fn new(position: Pos, velocity: Pos) -> Self {
        Bot { position, velocity }
    }

    // Method to update the position based on velocity
    pub fn update_pos(&mut self, wallx: Num, wally: Num) {
        let (x, y) = self.position;
        let (vx, vy) = self.velocity;
        self.position = (Bot::modulo_arithmetic(x, wallx, vx), Bot::modulo_arithmetic(y, wally, vy));
    }

    fn modulo_arithmetic(x: i32, modulus: i32, add: i32) -> i32 {
        return (x + add).rem_euclid(modulus);
        /*
        if result < 0 {
            result + modulus
        } else {
            result % modulus
        }
        */
    }
}

#[derive(Debug)]
struct Context {
    bots: Vec<Bot>
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        Context {
            bots: inp.into_iter().map(Context::parse_line).collect()
        }
    }

    fn parse_line(s: String) -> Bot {
        let t: Vec<&str> = s.split_ascii_whitespace().collect();
        let pos = t[0];
        let vel = t[1];
        let positions: Vec<Num> = pos
            .trim_start_matches("p=")
            .split(",")
            .map(|v| v.parse::<Num>().unwrap())
            .collect();
        let vels: Vec<Num> = vel.trim_start_matches("v=")
            .split(",")
            .map(|v| v.parse::<Num>().unwrap())
            .collect();
        Bot::new((positions[0], positions[1]), (vels[0], vels[1]))
    }

    fn quadrant_count(&self, wallx: Num, wally: Num) -> Num {
        let mx = wallx / 2;
        let my = wally / 2;
        let mut quadrants = [0,0,0,0];
        for b in &self.bots {
            // Axes
            if b.position.0 == mx || b.position.1 == my {
                continue;
            }
            let left = b.position.0 < mx;
            let top = b.position.1 < my;
            match (left, top) {
                (true, true) => quadrants[1] += 1,
                (false, true) => quadrants[0] += 1,
                (true, false) => quadrants[2] += 1,
                (false, false) => quadrants[3] += 1
            }
        }
        //println!("quandrant: {:?}", quadrants);
        quadrants.into_iter().fold(1, |acc, x| acc * x)
    }

    fn symmetry_score(&self, wallx: Num, wally: Num) -> usize {
        let mut sym_count = 0;
        let mx = wallx / 2;
        let my = wally / 2;
        let set: HashSet<(Num, Num)> = self.bots.iter().map(|b| b.position).collect();
        set.len()
    }

    pub fn part1(&mut self, N: usize, wallx: Num, wally: Num) -> Num {
        //println!("Start: {:?}", self);
        for i in 0..N {
            for b in &mut self.bots {
                b.update_pos(wallx, wally);
                //println!("{:?}", b);
            }
        }
        self.quadrant_count(wallx, wally)
    }

    pub fn part2(&mut self, N: usize, wallx: Num, wally: Num) {
        let mut map: HashMap<Num, Vec<usize>> = HashMap::new();
        let mut max_score = 0;
        for i in 0..N {
            for b in &mut self.bots {
                b.update_pos(wallx, wally);
            }
            let sym_score = self.symmetry_score(wallx, wally);
            if sym_score == 500 {
                println!("Iteration {}, score {}", i + 1, sym_score);
                self.plot("500");
                return;
            }
        }
    }

    fn plot(&self, name: &str) {
        // Define your points
        let points: Vec<(Num, Num)> = self.bots.iter().map(|b| b.position).collect();
    
        // Define the image dimensions
        let width: Num = 101;
        let height: Num = 103;
    
        // Create a new grayscale image
        let mut img = GrayImage::new(width as u32, height as u32);
    
        // Plot the points on the image
        for (x, y) in points {
            if x < width && y < height {
                img.put_pixel(x as u32, y as u32, image::Luma([255])); // White pixel for visibility
            }
        }

        println!("Finished plotting");
    
        // Save the image
        let filename = "/home/raghav/dev/aoc2024/src/aoc14/plot-".to_owned() + name + ".png";
        println!("path: {:?}", filename);
        img.save(filename).unwrap();
        println!("Saved image");
    }

}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc14 {
    use super::*;

//    #[test]
    fn example1() {
        let text: Vec<String> = vec!["p=2,4 v=2,-3".to_owned()];
        let mut context = Context::new(text);
        //let part1 = context.part1(5, 11, 7);
        //println!("Example part1: {}", part1);
        //let (part1, area) = context.part1();
        //println!("Example1 Part1: {:?}", part1);
        //println!("Example1 Part2: {:?}", context.part2());
    }

//    #[test]
    fn example2() {
        let text: Vec<String> = read_input("src/aoc14/example").expect("couldn't read input - aoc14");
        let mut context = Context::new(text);
        //println!("context: {:?}", context);
        //let part1 = context.part1(100, 11, 7);
        //println!("Example part1: {}", part1);
        //let (part1, area) = context.part1();
        //println!("Example1 Part1: {:?}", part1);
        //println!("Example1 Part2: {:?}", context.part2());
        //context.part2(1000, 11, 7);
    }

   #[test]
    fn actual() {
        // 222899040
        // 224583840
        let text: Vec<String> = read_input("src/aoc14/input").expect("couldn't read input - aoc14");
        let mut context = Context::new(text);
        //let part1 = bench(|| context.part1(100, 101, 103), Some("part1"));
        //println!("Part1: {:?}", part1);
        //println!("Part2: {:?}", bench(|| context.part2(), Some("part2")));
        bench(|| context.part2(100000, 101, 103), Some("Part2"));
    }
}