use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./assets/input") {
        let (mut c1, mut c2): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new()); 
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let (a, b) = line.split_once("   ").unwrap();
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            c1.push(a);
            c2.push(b);
        }

        c1.sort();
        c2.sort();

        // part1(c1, c2);
        part2(c1, c2);
        
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn part1(c1: Vec<i32>, c2: Vec<i32>) {
//     let mut sum = 0;
//     for i in 0..c1.len() {
//         sum += (c1[i] - c2[i]).abs();
//     }
//     println!("{}", sum);
// }

fn part2(c1: Vec<i32>, c2: Vec<i32>) {
    let mut idem_score: Vec<(i32, i32)> = Vec::new();
    // println!("comparing {:?} with {:?}", c1, c2);
    // let mut prev_index = 0;

    for i in 0..c1.len() {
        let score = c2.iter().filter(|&x| x == &c1[i]).count();
        idem_score.push((c1[i], score as i32));
    }
    
    let sum =  idem_score.iter().fold(0, |acc, x| acc + x.0 * x.1);

    println!("{}", sum);
}

