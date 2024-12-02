use crate::utils::read_input;

#[derive(Debug)]
struct Context {
    levels: Vec<Vec<i32>>
}

impl Context {
    pub fn new(inp: Vec<String>) -> Context {
        Context {
            levels: inp.iter()
                .map(|s| s.split_whitespace()
                    .map(|t| t.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>())
                .collect()
        }
    }

    fn check_safety(l: &Vec<i32>, list_len: usize) -> (bool, usize, usize) {
        let inc: bool = l[1] - l[0] > 0;
        for i in 0..(l.len() - 1) {
            let diff = l[i + 1] - l[i];
            let ord = diff > 0;
            let abs_diff = diff.wrapping_abs();
            if inc ^ ord || abs_diff < 1 || abs_diff > 3 {
                return (false, i, i + 1);
            }
        }
        (true, usize::MAX, usize::MAX)
    }

    pub fn part1(&self) -> usize {
        let list_len = self.levels[0].len();
        self.levels.iter()
            .map(|l| Context::check_safety(l, list_len))
            .filter(|&x| x.0)
            .count()
    }

    fn remove(vec: &Vec<i32>, index: usize) -> Vec<i32> {
        let mut first = match index == 0 {
            true => vec![],
            false => vec[0..index].to_vec()
        };
        let second = match index == vec.len() - 1 {
            true => vec![],
            false => vec[(index+1)..].to_vec()
        };
        first.extend(second);
        first
    }

    pub fn part2(&self) -> usize {
        let list_len = self.levels[0].len();
        self.levels.iter()
            .map(|l| {
                let part1 = Context::check_safety(l, list_len);
                if part1.0 {
                    return true;
                }
                for i in 0..l.len() {
                    let option = Context::remove(l, i);
                    if Context::check_safety(&option, list_len).0 {
                        return true;
                    }
                }
                false
            })
            .filter(|&x| x)
            .count()
    }
}



#[cfg(test)]
mod aoc2 {
    use super::*;

    #[test]
    fn example1() {
        let text: Vec<String> = read_input("src/aoc2/example").expect("couldn't read input - aoc2");
        let context = Context::new(text);
        println!("Part1: {:?}", context.part1());
        println!("Part2: {:?}", context.part2());
    }

    #[test]
    fn actual() {
        let text: Vec<String> = read_input("src/aoc2/input").expect("couldn't read input - aoc2");
        let context = Context::new(text);
        println!("Part1: {:?}", context.part1());
        println!("Part2: {:?}", context.part2());
    }
}