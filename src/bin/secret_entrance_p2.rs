use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const LEFT: &str = "L";
const RIGHT: &str = "R";

fn main() {
    if let Ok(lines) = read_lines("src/input/d1_p2.txt") {
        let mut dial: i32 = 50;
        let mut left_zero: i32 = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            //parse the value
            let (index, count) = line.split_at(1);
            let mut value = count.parse::<i32>().unwrap();
            //if it passes 0 when rotate to L or R count++
            match index {
                LEFT => {
                    if value > 100 {
                        let add_to_count = value / 100;
                        left_zero += add_to_count;
                        value = value % 100;
                    }
                    let dif = dial - value;
                    if dif < 0 {
                        if dial != 0 {
                            left_zero += 1;
                        }
                        dial = 100 + dif
                    } else {
                        dial -= value;
                    }

                    if dial == 100 {
                        dial = 0;
                    }
                    if dial == 0 {
                        left_zero += 1;
                    }
                    println!("The dial is rotated {index}{value} to point at {dial}.");
                }
                RIGHT => {
                    if value > 100 {
                        let add_to_count = value / 100;
                        left_zero += add_to_count;
                        value = value % 100;
                    }
                    let dif = dial + value;
                    if dif > 100 {
                        left_zero += 1;
                        dial = dif % 100
                    } else {
                        dial = dif
                    }
                    if dial == 100 {
                        dial = 0;
                    }
                    if dial == 0 {
                        left_zero += 1;
                    }
                    println!("The dial is rotated {index}{value} to point at {dial}.");
                }
                _ => {
                    println!("not L or R")
                }
            }
        }
        println!("counts:{left_zero}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
