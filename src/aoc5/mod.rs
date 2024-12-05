use crate::utils::{read_input, bench};
use std::{cmp::Ordering, collections::{HashMap, HashSet}, ops::Index};

#[derive(Debug)]
struct Context {
    vertices: HashSet<u32>,
    edges: HashMap<u32, Vec<u32>>,
    reverse_edges: HashMap<u32, Vec<u32>>,
    page_orderings: Vec<Vec<u32>>,
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        let parse_u32 = |s: &str| { s.parse::<u32>().unwrap() };
        let mut i = 0;
        let mut vertices = HashSet::new();
        let mut edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut reverse_edges: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut page_orderings = vec![];
        loop {
            if inp[i].trim().is_empty() {
                i += 1;
                break;
            }
            let edge: Vec<u32> = inp[i].split('|').into_iter().map(|s| parse_u32(s)).collect();
            edges.entry(edge[0]).and_modify(|c: &mut Vec<u32>| (*c).push(edge[1])).or_insert(vec![edge[1]]);
            reverse_edges.entry(edge[1]).and_modify(|c: &mut Vec<u32>| (*c).push(edge[0])).or_insert(vec![edge[0]]);
            i += 1;
        }
        while i < inp.len() {
            let verts: Vec<u32> = inp[i].split(',').into_iter().map(parse_u32).collect();
            vertices.extend(verts.iter());
            page_orderings.push(verts);
            i += 1;
        }
        Context {
            vertices: vertices,
            edges: edges,
            reverse_edges: reverse_edges,
            page_orderings: page_orderings
        }
    }

    pub fn part1(&self) -> (u32, Vec<usize>) {
        let mut sum = 0;
        let mut correct = vec![];
        for (k, ordering) in self.page_orderings.iter().enumerate() {
            let mut flag = true;
            for i in 0..ordering.len() {
                for j in i + 1..ordering.len() {
                    if self.edges.contains_key(&ordering[j]) && self.edges.get(&ordering[j]).unwrap().contains(&ordering[i]) {
                        flag = false;
                        break;
                    }
                }
                if !flag {
                    break;
                }
            }
            if flag {
                correct.push(k);
                sum += ordering[ordering.len() / 2];
            }
        }
        (sum, correct)
    }

    // Not used. Overall graph is not a DAG - has cycles
    fn topo_sort(&self, node: &u32, permanent: &mut HashSet<u32>, temporary: &mut HashSet<u32>, sorted_list: &mut Vec<u32>) {
        println!("Visting node {}", node);
        if permanent.contains(node) {
            return;
        }
        if temporary.contains(node) {
            println!("Temporary: {:?}", temporary);
            println!("Permanent: {:?}", permanent);
            println!("sorted_list: {:?}", sorted_list);
            panic!("Graph has a cycle");
        }
        temporary.insert(*node);
        let egdes = self.reverse_edges.get(node);
        match egdes {
            Some(edge) => {
                for e in edge {
                    self.topo_sort(e, permanent, temporary, sorted_list);
                }
            },
            None => (),
        }
        temporary.remove(node);
        permanent.insert(*node);
        sorted_list.push(*node);
    }

    fn part2(&self, correct_list: &Vec<usize>) -> u32 {
        let mut sum = 0;
        for (k, ordering) in self.page_orderings.iter().enumerate() {
            if correct_list.contains(&k) {
                continue;
            }
            let mut new_ordering = ordering.clone();
            let mut i = 0;
            while i < new_ordering.len() {
                let mut num_swaps = 0;
                for j in i+1..new_ordering.len() {
                    if self.edges.contains_key(&new_ordering[j]) && self.edges.get(&new_ordering[j]).unwrap().contains(&new_ordering[i]) {
                        let tmp = new_ordering[j];
                        new_ordering[j] = new_ordering[i];
                        new_ordering[i] = tmp;
                        num_swaps += 1;
                        break;
                    }
                }
                if num_swaps == 0 {
                    i += 1;
                }
            }
            sum += new_ordering[new_ordering.len() / 2];
        }
        sum
    }
}


#[cfg(test)]
mod aoc5 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc5/example").expect("couldn't read input - aoc5");
        let context = Context::new(text);
        //println!("context: {:?}", context);
        let (part1, correct) = context.part1();
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", context.part2(&correct));
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc5/input").expect("couldn't read input - aoc5");
        let context = Context::new(text);
        let (part1, correct) = bench(|| context.part1(), Some("part1"));
        println!("Part1: {:?}", part1);
        println!("Part2: {:?}", bench(|| context.part2(&correct), Some("part2")));
    }
}