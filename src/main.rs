use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Ok(lines) = read_lines(&args[1]) {
        let mut board :[[u8; 9]; 9] = [[0; 9]; 9];

        for (row, line) in lines.enumerate() {
            if let Ok(ln) = line {
                let chunks: Vec<_> = ln.split_whitespace().collect();
                for (column, chunk) in chunks.into_iter().enumerate() {
                    println!(";;;{}, ", chunk.parse::<u8>().unwrap());
                    board[row][column] = chunk.parse::<u8>().unwrap();
                }
            }
        }
        
        for row in 0..9 {
            for col in 0..9 {
                println!("({},{}) = {}", row, col, board[row][col]);
            }
        } 
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
        let file = fs::File::open(filename).expect("Unable to open file!");
        Ok(io::BufReader::new(file).lines())
    }