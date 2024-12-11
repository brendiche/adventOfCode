
fn main() {
    let lines = std::fs::read_to_string("./assets/input").unwrap();
    let  matrix = lines.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    // part1(&matrix.clone());
    part2(&matrix)
    
}

fn part2(matrix: &Vec<Vec<char>>) {
    let a_position = get_char_positions(&matrix, 'A');
    // println!("{:?}", a_position);
    println!("number of a {}", a_position.len());
    let xmas = get_ms(matrix, &a_position);
    println!("number of xmas {}", xmas)
}

// fn part1(matrix: &Vec<Vec<char>>){
//     let x_position = get_char_positions(&matrix, 'X');
//     // println!("first X : {:?}", x_position[0]);
//     // println!("number of x {:?}", x_position.len()w);
//     let xm_position = get_xm_positions(&matrix, &x_position);
//     // println!("first XM : {:?} , {:?}", xm_position[0], xm_position[1]);
//     println!("number of xm {:?}", xm_position.len());
//     let xmas = get_xmas(&matrix, &xm_position);
//     println!("number of xmas {}", xmas);
// }


fn get_char_positions(matrix: &Vec<Vec<char>>, character: char) -> Vec<(usize, usize)> {
    let mut position: Vec<(usize, usize)> = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == character {
                position.push((i, j));
            }
        }
    }
    position
}

fn get_ms(matrix: &Vec<Vec<char>>, a_positions: &Vec<(usize, usize)>) -> i32 {
    let mut count = 0;
    a_positions.iter().for_each(|a_position| {
        let mut m_count = 0;
        let mut s_count = 0;
        let scan = Vec::from([(-1,-1),(-1,1),(1,-1),(1,1)]);
        if  a_position.0 < matrix.len() - 1 &&
            a_position.0 > 0 &&
            a_position.1 < matrix[a_position.0].len() - 1 &&
            a_position.1 > 0
        { 
        // println!("### for a {:?} ###", a_position);
        scan.iter().for_each(|letter_candidate|{
                let row_candidate = matrix.get((a_position.0 as i32 + letter_candidate.0) as usize );
                match row_candidate {
                    Some(row) => {
                        let cel_candidate = row.get((a_position.1 as i32 + letter_candidate.1) as usize );
                        match cel_candidate {
                            Some(cel) => {
                                // println!("cel position {:?}, with content {}",(a_position.0 as i32 + letter_candidate.0, a_position.1 as i32 + letter_candidate.1), cel);
                                match cel{
                                    'M' => m_count += 1, 
                                    'S' => s_count += 1,
                                    _ =>()
                                }
                            }
                            None => ()
                        }
                    }
                    None => ()
                }
            });
        }
        // println!("for a {:?} m_count {} and s_count {}", a_position, m_count, s_count);
        if m_count == 2 && s_count == 2 { 
            // remove corner cases
            if matrix[(a_position.0 as i32 +scan[0].0) as usize][(a_position.1 as i32 +scan[0].1) as usize] != matrix[(a_position.0 as i32 +scan[3].0) as usize][(a_position.1 as i32 +scan[3].1) as usize]{
                // println!("valid xmas => {:?}", a_position);
                count += 1 
            }
        }
        // println!("")

    });
    count
}

// fn get_xm_positions(matrix: &Vec<Vec<char>>, x_positions: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
//     let mut position: Vec<((usize, usize), (usize, usize))> = Vec::new();
//     x_positions.iter().for_each(|x_position: &(usize, usize)| {
//         // remove the unreachable positions check

//         for scan_i in -1..2 {
//             for scan_j in -1..2 {
//                 // println!("scan {} {}", scan_i, scan_j);
//                 if  (x_position.0 as i32 + scan_i) < matrix.len() as i32 &&
//                     (x_position.0 as i32 + scan_i) >= 0 &&
//                     (x_position.1 as i32 + scan_j) < matrix[x_position.0].len() as i32 &&
//                     (x_position.1 as i32 + scan_j) >= 0
//                  {
//                     // println!("allow scan for posisiton {} {}", x_position.0 as i32 + scan_i, x_position.1 as i32 + scan_j);
//                     // if x_position.0 == 0 && x_position.1 == 4 {
//                     //     println!("scan {} {} {}", scan_i, scan_j, matrix[(x_position.0 as i32 + scan_i) as usize][(x_position.1 as i32 + scan_j) as usize]);
//                     // }
//                     if matrix[(x_position.0 as i32 + scan_i) as usize][(x_position.1 as i32 + scan_j) as usize] == 'M' {
//                         position.push((*x_position, ((x_position.0 as i32 + scan_i) as usize, (x_position.1 as i32 + scan_j) as usize)));
//                     }
//                 } 
//             }
//         }
//     });
//     position
// }

// fn get_xmas(matrix: &Vec<Vec<char>>, xm_positions: &Vec<((usize, usize), (usize, usize))>) -> i32 {
//     let mut count=0;
//     xm_positions.iter().for_each(|xm_position: &((usize, usize), (usize, usize))| {
//         let direction_vector = get_direction(xm_position.0, xm_position.1);
//         let candidate_row = matrix.get((xm_position.1.0 as i32 + direction_vector.0 as i32) as usize);
//         match candidate_row {
//             Some(row) => {
//                 let candidate_cel = row.get((xm_position.1.1 as i32 + direction_vector.1 as i32) as usize);
//                 match candidate_cel {
//                     Some(cel) => {
//                         if *cel == 'A' {
//                             let candidate_row = matrix.get((xm_position.1.0 as i32 + 2*direction_vector.0 as i32) as usize);
//                             match candidate_row {
//                                 Some(row) => {
//                                     let candidate_cel = row.get((xm_position.1.1 as i32 + 2*direction_vector.1 as i32) as usize);
//                                     match candidate_cel {
//                                         Some(cel) => {
//                                             if *cel == 'S' {
//                                                 count += 1
//                                             }
//                                         }
//                                         None => ()
//                                     }
//                                 }
//                                 None => ()
//                             }
//                         }
//                     }
//                     None => ()
//                 }
//             }
//             None => ()
//         }
//     });
//     count
// }

// fn get_direction(x:(usize, usize), m:(usize, usize)) -> (i32, i32) {
//     // println!("x :{:?}, m :{:?}", x, m);
//      ((m.0 as i32 - x.0 as i32), (m.1 as i32 - x.1 as i32))
// }