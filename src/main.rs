use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
enum Value {
    Variable {val: u8, domain: Vec<u8>}, 
    Constant(u8),
    Fail // A search has failed: inform the solver to quit
}

fn print_board(board: &Vec<Value>) {
    println!("Size of board init: {}", board.len());
    for row in 0..9 {
        for col in 0..9 {
            print!("({},{}) == ", row, col);
            match &board[row + col*9] {
                Value::Fail => println!("{}",  "Fail!"),
                Value::Constant(num) => println!("CONSTANT {}", num),
                Value::Variable{val, domain} => println!("{}", val)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board: Vec<Value> = Vec::new();
    board.push(Value::Fail);

    if let Ok(lines) = read_lines(&args[1]) {

        for (row, line) in lines.enumerate() {
            if let Ok(ln) = line {
                let chunks: Vec<_> = ln.split_whitespace().collect();
                for (column, chunk) in chunks.into_iter().enumerate() {
                    let digit = chunk.parse::<u8>().unwrap();
                    
                    if digit == 0 {
                        // We have an undefined variable!
                        board.push(Value::Variable {val: 0, domain: vec![1, 2, 3, 4, 5, 6, 7, 8, 9]});
                    } else {
                        // We have a constant Value
                        board.push(Value::Constant(digit));
                    }
                }
            }
        }
        
        print_board(&board);
        
    }
}

impl Value {
    fn get_value(&self) -> u8 {
        match self {
            Value::Fail => 0,
            Value::Constant(x) => *x,
            Value::Variable{val, domain} => *val
        }
    }
}

// Check if a given grid cell is unique in a row
fn unique_in_row(board: &Vec<Value>, row: usize, var: Value) -> bool {
    let mut occurence = 0;
    for col in row..row+9 {
        if board[row + col*9+1].get_value() == var.get_value() {
            occurence += 1;
        }
    }
    occurence == 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
        let file = fs::File::open(filename).expect("Unable to open file!");
        Ok(io::BufReader::new(file).lines())
    }