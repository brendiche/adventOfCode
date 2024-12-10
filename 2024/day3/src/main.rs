use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
    let lines = read_lines("./assets/input").unwrap();
    for line in lines {
        let line = line.unwrap();
        // part1(line.clone())
        part2(line.clone())
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part2(line: String) {
    let scan_string = line.split("").collect::<Vec<_>>();
    let mut mul_enabled = true;
    let mut sum = 0;
    //  print!("{:?}\n", scan_string);
    for i in  1..scan_string.len()-1 {
        if scan_string[i] == "(" {
            let modifier = scan_string.get(i+1);
            match modifier {
                Some(modifier) => {
                    if modifier.eq(&")") {
                        let action = scan_string.get((i-(if mul_enabled  {5} else {2}))..i);
                        match action {
                            Some(action) => {
                                let action = action.join("");
                                if action.eq("don't") {
                                    mul_enabled = false;
                                    // println!("action: {:?}", action);
                                } else if action.eq("do")  {
                                    mul_enabled = true;
                                    // println!("action: {:?}", action);

                                }
                            }

                            None => ()
                        }
                    } else if mul_enabled {
                        let mul_instruction = scan_string.get((i-3)..i);
                        match mul_instruction {
                            Some(mul_instruction) => {
                                let mul_instruction = mul_instruction.join("");
                                if mul_instruction.eq("mul") {
                                    // println!("mul_instruction: {}", mul_instruction);
                                    let digits = scan_string.get((i+1)..(if i+9 > scan_string.len() {scan_string.len()} else {i+9}));
                                    match digits {
                                        Some(digits) => {
                                            let digits = digits.join("");
                                            // println!("digits: {}", digits);
                                            let digits = digits.split(")").collect::<Vec<_>>();
                                            let digits = digits[0].split(",").collect::<Vec<_>>();
                                            let a = digits.get(0).unwrap().parse::<i32>();
                                            match a {
                                                Ok(a) => {
                                                    // println!("a: {}", a);
                                                    let b = digits.get(1);
                                                    match b {
                                                        Some(b) => {
                                                            let b = b.parse::<i32>();
                                                            match b {
                                                                Ok(b) => {
                                                                    // println!("a: {}, b: {}", a, b);
                                                                    sum += a * b;
                                                                },
                                                                Err(_) => ()
                                                            }
                                                        }
                                                        None => ()
                                                    }
                                                }
                                                Err(_) => ()
                                            }
                                        }

                                        None => ()
                                        
                                    }
                                }
                            }

                            None => ()
                        }
                    }
                }
                None => (),
            }
        }
    }
    println!("sum: {}", sum);
}


// fn part1(line: String) {
//     // println!("{}", line);
//         // parse the line on "mul(" so it scan every character
//         let start_mul = line.split("mul(").collect::<Vec<_>>();
//         // println!("{:?}", scan);
//         // iterate on the vec to find candidates
//         let mut sum = 0;
//         start_mul.iter()
//             .for_each(|potiential_mul| {
//                 // println!("{}", c);
//                 let candidates = potiential_mul.split(")").collect::<Vec<_>>();
//                 // println!("candidates: {:?}", candidates);
//                 let mul = candidates[0];
//                 if mul.len() >= 3 && mul.len() <= 7 {
//                     // println!("mul: {:?}", mul);
//                     let values = mul.split(",").collect::<Vec<_>>();
//                     let a = values.get(0).unwrap().parse::<i32>();
//                     let b = values.get(1);
//                     match  b {
//                         Some(b) => {
//                             let b = b.parse::<i32>();
//                             match a {
//                                 Ok(a) => { 
//                                     match b {
//                                         Ok(b) => sum += a * b,
//                                         Err(_) => ()
//                                     }
                                    
//                                 }
//                                 Err(_) => (),
//                             }
//                         }
//                         None => ()
//                     }

//                     // println!("a: {}, b: {}", a, b);
//                 }
//             });
//         println!("sum: {}", sum);
// }