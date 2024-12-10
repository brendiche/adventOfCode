use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    let lines = read_lines("./assets/input").unwrap();
    for line in lines {
        let line = line.unwrap();
        // println!("{}", line);
        // parse the line on "mul(" so it scan every character
        let start_mul = line.split("mul(").collect::<Vec<_>>();
        // println!("{:?}", scan);
        // iterate on the vec to find candidates
        let mut sum = 0;
        start_mul.iter()
            .for_each(|potiential_mul| {
                // println!("{}", c);
                let candidates = potiential_mul.split(")").collect::<Vec<_>>();
                // println!("candidates: {:?}", candidates);
                let mul = candidates[0];
                if mul.len() >= 3 && mul.len() <= 7 {
                    // println!("mul: {:?}", mul);
                    let values = mul.split(",").collect::<Vec<_>>();
                    let a = values.get(0).unwrap().parse::<i32>();
                    let b = values.get(1);
                    match  b {
                        Some(b) => {
                            let b = b.parse::<i32>();
                            match a {
                                Ok(a) => { 
                                    match b {
                                        Ok(b) => sum += a * b,
                                        Err(_) => ()
                                    }
                                    
                                }
                                Err(_) => (),
                            }
                        }
                        None => ()
                    }

                    // println!("a: {}, b: {}", a, b);
                }
            });
        println!("sum: {}", sum);
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}