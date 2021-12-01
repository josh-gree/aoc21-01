use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let fname = "/Users/josh-gree/aoc21/aoc21-01-a/data/aoc1a.txt".to_string();
    if let Ok(input) = parse_input_aoc1a(&fname) {
        let resa = aoc1a(input);
        println!("aoc1a result = {}", resa); 
    }

    if let Ok(input) = parse_input_aoc1a(&fname) {
        let resb = aoc1b(input);
        println!("aoc1b result = {}", resb); 
    }

}

fn aoc1a(input: Vec<i32>) -> usize {
    let seconds = &input[1..];
    let firsts = &input[..input.len()];

    let res = firsts.iter()
                        .zip(seconds)
                        .map(|(first,second)| (first < second) as usize)
                        .sum();

    res
}

fn aoc1b(input: Vec<i32>) -> usize {

    let window_len = 3;

    let windowed = input.windows(window_len).map(|slice| slice.iter().sum()).collect();

    aoc1a(windowed)
}

fn parse_input_aoc1a(fname: &String) -> Result<Vec<i32>,String> {
    
    let mut out = vec![];
    if let Ok(f) = File::open(fname) {
        let lines = io::BufReader::new(f).lines();
        for line in lines {
            if let Ok(l) = line {
                if let Ok(v) = l.parse::<i32>() {
                    out.push(v)
                } else {
                    return Err("Unable to parse number".to_string())
                }
            } else {
                return Err("Unable to read line".to_string())
            }
        }
    } else {
        return Err("Unable to open file".to_string())
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::aoc1a;
    use crate::parse_input_aoc1a;
    #[test]
    fn test_small_input() {
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        let output = aoc1a(input);

        assert_eq!(output, 7)

    }

    #[test]
    fn test_input_parse() {
        let fname = "/Users/josh-gree/aoc21/aoc21-01-a/data/aoc1a.txt".to_string();

        match parse_input_aoc1a(fname) {
            Ok(_input) => {},
            Err(error) => {panic!("{}", error)}
        }
    }
}
