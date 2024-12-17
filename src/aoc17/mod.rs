use crate::utils::{read_input, bench};
use core::num;
use std::fmt::{write, Debug, Formatter, Result};
use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::hash::Hash;
use std::ops::Index;
use std::usize;
use rayon::prelude::*;

type Num = u64;
type NumInd = usize;

#[derive(Debug)]
struct Context {
    registers: Vec<Num>,
    instructions: Vec<Num>
}

impl Context {
    pub fn new(registers: Vec<Num>, instructions: Vec<Num>) -> Context {
        Context {
            registers: registers,
            instructions: instructions
        }
    }

    fn get_operand(&self, operand_code: &Num) -> Num {
        if *operand_code < 4 {
            return *operand_code;
        }
        if *operand_code < 7 {
            return self.registers[(*operand_code - 4) as NumInd];
        }
        panic!("Operand code greater than 6: {}", operand_code);
    }

    pub fn part1(&mut self) -> String {
        let mut out: Vec<Num> = vec![];
        let mut i_point: NumInd = 0;
        let mut counter = 0;
        while i_point < self.instructions.len() {
            //println!("Current state");
            //println!("Registers: {:?}", self.registers);
            //println!("Output: {:?}", out);
            //println!("Instruction Index: {}", i_point);
            
            if counter > 100 {
                println!("Counter expired: out");
                return  out.iter().map(|v: &Num| v.to_string()).collect::<Vec<String>>().join(",");
            }
            let operation = self.instructions[i_point];
            let operand_ind = i_point + 1;
            let mut incr_by_2 = true;
            match operation {
                0 => {
                    let num = self.registers[0];
                    let den = (2 as Num).pow(u32::try_from(self.get_operand(&self.instructions[operand_ind])).unwrap());
                    self.registers[0] = num / den;
                },
                1 => {
                    let n1 = self.registers[1];
                    let n2 = self.instructions[operand_ind];
                    self.registers[1] = n1 ^ n2;
                },
                2 => {
                    let op = self.get_operand(&self.instructions[operand_ind]);
                    self.registers[1] = op % 8;
                },
                3 => {
                    if self.registers[0] != 0 {
                        i_point = self.instructions[operand_ind] as NumInd;
                        incr_by_2 = false;
                    }
                },
                4 => {
                    let n1 = self.registers[1];
                    let n2 = self.registers[2];
                    self.registers[1] = n1 ^ n2;
                },
                5 => {
                    let o = self.get_operand(&self.instructions[operand_ind]);
                    out.push(o % 8);
                },
                6 => {
                    let num = self.registers[0];
                    let den = (2 as Num).pow(u32::try_from(self.get_operand(&self.instructions[operand_ind])).unwrap());
                    self.registers[1] = num / den;
                },
                7 => {
                    let num = self.registers[0];
                    let den = (2 as Num).pow(u32::try_from(self.get_operand(&self.instructions[operand_ind])).unwrap());
                    self.registers[2] = num / den;
                },
                _ => panic!("Unknown instruction"),
            };
            if incr_by_2 {
                i_point += 2;
            }
            counter += 1;
        }
        //println!("Counter: {}", counter);
        out.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
    }

    fn bfs(&self) {
        let cur_a = 0;
        let mut q = VecDeque::new();
        let mut v = self.instructions.clone();
        v.reverse();
        q.push_back((cur_a, 0));

        let mut min_val = Num::MAX;

        while !q.is_empty() {
            let (cur, ind) = q.pop_front().unwrap();
            if ind == v.len() {
                min_val = min_val.min(cur);
                continue;
            }
            let start = 8 * cur;
            let end = start + 8;
            for j in start..end {
                let tmp = self.compute_expression(j);
                if tmp == v[ind] {
                    q.push_back((j, ind + 1));
                }
            }
        }
        println!("min_val: {}", min_val);
    }

    fn compute_test(&self, A: Num) -> Num {
        (A / 8) % 8
    }

    fn compute_expression(&self, A: Num) -> Num {
        // Calculate the first XOR operation
        let first_xor = (A % 8) ^ 3;
    
        // Calculate the inner exponentiation and XOR operation
        let inner_xor = (A % 8) ^ 5;
        // Since 2**inner_xor will be very large, we assume A/2**inner_xor is effectively 0 for most practical purposes
        // However, to be precise, we calculate it as follows:
        let second_xor = A / ((2 as Num).pow(inner_xor as u32));
    
        // Perform the second XOR operation
        let result = first_xor ^ second_xor;
    
        // Apply the final modulus operation
        let final_result = result % 8;
    
        final_result
    }

    pub fn part2(&mut self) -> Vec<Num> {
        let mut out = vec![];
        let mut i_point: NumInd = 0;
        let mut counter = 0;
        while i_point < self.instructions.len() {
            // println!("Current state");
            // println!("Registers: {:?}", self.registers);
            // println!("Output: {:?}", out);
            // println!("Instruction Index: {}", i_point);
            if counter > 100 {
                println!("Counter expired: out");
                return out;
            }
            let operation = self.instructions[i_point];
            let operand_ind = i_point + 1;
            let mut incr_by_2 = true;
            match operation {
                0 => {
                    let num = self.registers[0];
                    let den = (2 as Num).pow(u32::try_from(self.get_operand(&self.instructions[operand_ind])).unwrap());
                    self.registers[0] = num / den;
                },
                1 => {
                    let n1 = self.registers[1];
                    let n2 = self.instructions[operand_ind];
                    self.registers[1] = n1 ^ n2;
                },
                2 => {
                    let op = self.get_operand(&self.instructions[operand_ind]);
                    self.registers[1] = op % 8;
                },
                3 => {
                    if self.registers[0] != 0 {
                        i_point = self.instructions[operand_ind] as NumInd;
                        incr_by_2 = false;
                    }
                },
                4 => {
                    let n1 = self.registers[1];
                    let n2 = self.registers[2];
                    self.registers[1] = n1 ^ n2;
                },
                5 => {
                    let o = self.get_operand(&self.instructions[operand_ind]);
                    out.push(o % 8);
                },
                6 => {
                    let num = self.registers[0];
                    let den = (2 as Num).pow(u32::try_from(self.get_operand(&self.instructions[operand_ind])).unwrap());
                    self.registers[1] = num / den;
                },
                7 => {
                    let num = self.registers[0];
                    let den = (2 as Num).pow(u32::try_from(self.get_operand(&self.instructions[operand_ind])).unwrap());
                    self.registers[2] = num / den;
                },
                _ => panic!("Unknown instruction"),
            };
            if incr_by_2 {
                i_point += 2;
            }
            counter += 1;
        }
        //println!("Counter: {}", counter);
        out
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod aoc17 {
    use super::*;

//    #[test]
    fn example1() {
        //let text: Vec<String> = read_input("src/aoc17/example1").expect("couldn't read input - aoc17");;
        let mut context = Context::new(vec![729, 0, 0], vec![0,1,5,4,3,0]);
        let part1 = context.part1();
        println!("Example1 Part1: {:?}", part1);
        //println!("Example1 Part2: {:?}", context.part2(&cost));
    }

//    #[test]
    fn example2() {
        //let text: Vec<String> = read_input("src/aoc17/example1").expect("couldn't read input - aoc17");;
        let mut context = Context::new(vec![117440, 0, 0], vec![0,3,5,4,3,0]);
        //let part1 = context.part1();
        //println!("Example1 Part1: {:?}", part1);
        //println!("Example1 Part2: {:?}", context.part2());

        // let mut a_prev = 0;
        // let mut v = vec![0,3,5,4,3,0];
        // v.reverse();
        // for i in 0..v.len() {
        //     let start = 8 * a_prev;
        //     let end = start + 8;
        //     let mut found = false;
        //     for j in start..end {
        //         let x = context.compute_expression(j);
        //         if x == v[i] {
        //             found = true;
        //             a_prev = j;
        //         }
        //     }
        //     if !found {
        //         println!("Not found");   
        //     }
        //     println!("For iteration {}, val: {}", i, a_prev);
        // }
        context.bfs();
        
    }

   #[test]
    fn actual() {
        //let text: Vec<String> = read_input("src/aoc17/input").expect("couldn't read input - aoc17");
        let mut context = Context::new(vec![44348299, 0, 0], vec![2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0]);
        let part1 = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        // (((A%8)^3) ^ (A/2**((A%8)^5))) % 8 -> Analytical equation that program computes
        // The value of A keeps decreasing until it reaches 0. At each step, the %8 basically means that the last 3 bits are removed.
        // The insight is to compute this expression in reverse starting from the last value output (0) to the first value(2) 
        // So going in the opposite direction, we add back 3 bits i.e left shift
        // But the value of the 3 bits could be anything - (from 0 to 7)
        // So say at step 0, we check from 0 to 7 and find that A=3 provides 0 as the answer.
        // For step 1, we need to consider all numbers from 3 * 8 to 3 * 8 + 7. Note that there could be multiple possibilities in each step
        // So we do a DFS/BFS until we reach the end of the instruction list.

        bench(|| context.bfs(), Some("part2"));
        //let mut context = Context::new(vec![2097154, 0, 0], vec![2,4,1,5,7,5,1,6,0,3,4,2,5,5,3,0]);
        //println!("For index 2097154 -> {}", context.part1());
        //println!("Part2: {:?}", bench(|| context.part2(&cost), Some("part2")));
    }
}