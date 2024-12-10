use std::{fs::File, io::{self, BufRead, BufReader, Lines}, path::Path};



fn main() {
    if let Ok(reports) = read_lines("./assets/input") {
        // part1(reports);
        part2(reports);
    }
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part2(reports: Lines<BufReader<File>>) {
    let mut safe_reports = 0;
    for report in reports.flatten() {
        let mut unsafe_flag = false;
        let levels =report.split(" ");
        let mut levels = levels
            .into_iter()
            .map(|level| level.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let increase = is_report_increasing(levels.clone());
        for i in 1..(levels.len()) {
            let result = levels[i] - levels[i-1];
            //  increasing or decreasing condition
            if  increase &&( result <= 0) || 
                !increase &&( result >= 0) || 
                // adjacent condition
                result.abs() < 1 || result.abs() > 3 {
                    println!("\nunsafe report: {}, error on index: {}", report, i);
                    let mut sub_report_i = levels.clone();
                    let mut sub_report_i_minus_one = levels.clone();
                    sub_report_i.remove(i); 
                    sub_report_i_minus_one.remove(i-1);
                    if !is_sub_report_safe(sub_report_i.clone()) && !is_sub_report_safe(sub_report_i_minus_one.clone()) {
                        unsafe_flag = true;
                    }
                    break;
            }
        }

        if !unsafe_flag {
            println!("safe report: {}", report);
            safe_reports += 1;
        }
    }

    println!("number of safe reports: {}", safe_reports);
}

fn is_report_increasing(levels: Vec<i32>) -> bool {
    let mut increase_count = 0;
    for i in 1..(levels.len()) {
        let result = levels[i] - levels[i-1];
        if result > 0 {
            increase_count += 1;
        }                
    }

    increase_count > (levels.len() - 1 )/ 2
}

fn is_sub_report_safe(levels: Vec<i32>) -> bool {
    let mut increase = true;
    let mut unsaf_flag = false;
    for i in 1..(levels.len()) {
        let result = levels[i] - levels[i-1];
        if i == 1 {
            increase = result > 0;                
        }

        //  increasing or decreasing condition
        if  increase &&( result <= 0) || 
            !increase &&( result >= 0) || 
            // adjacent condition
            result.abs() < 1 || result.abs() > 3 {
                println!("\t unsafe sub report: {:?}, error on index: {}", levels, i);
                unsaf_flag = true;
                break;
        }
    }
    !unsaf_flag
}

// fn part1(reports: Lines<BufReader<File>>) {
//     let mut safe_reports = 0;
//     for report in reports.flatten() {
//         let mut unsaf_flag = false;
//         let mut increase = true;
//         let levels =report.split(" ");
//         let levels = levels
//             .into_iter()
//             .map(|level| level.parse::<i32>().unwrap())
//             .collect::<Vec<i32>>();

//         for i in 1..(levels.len()) {
//             let result = levels[i] - levels[i-1];
//             if i == 1 {
//                 increase = result > 0;                
//             }

//             //  increasing or decreasing condition
//             if  increase &&( result <= 0) || 
//                 !increase &&( result >= 0) || 
//                 // adjacent condition
//                 result.abs() < 1 || result.abs() > 3{
//                     unsaf_flag = true;
//                     break;
//             }
//         }

//         if !unsaf_flag {
//             safe_reports += 1;
//         }
//     }

//     println!("number of safe reports: {}", safe_reports);
// }