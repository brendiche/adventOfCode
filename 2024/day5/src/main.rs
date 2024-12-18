fn main() {
    let input = std::fs::read_to_string("./assets/input").unwrap();
    let input_parts = input.split("\n\n").collect::<Vec<_>>();
    let mut rule_before = Vec::new();
    let mut rule_after = Vec::new();
    input_parts[0].lines().collect::<Vec<_>>().iter().for_each(|rule| {
        let rule_parts = rule.split("|").collect::<Vec<_>>();
        rule_before.push(rule_parts[0]);
        rule_after.push(rule_parts[1]);
    });
    let page_order =  input_parts[1].lines().collect::<Vec<_>>();
    part2(page_order, rule_before, rule_after);
    // part1(page_order, rule_before, rule_after);
    // println!("rules_before {:?}", rule_before);
    // println!("rules_after {:?}", rule_after);
    // println!("page order {:?}", page_order);
}

fn part2(page_order: Vec<&str>, rule_before: Vec<&str>, rule_after: Vec<&str>){
    let mut sum = 0;
    page_order.iter().for_each(|page| {
    let mut pages_numbers = page.split(",").map(|f| f.to_string()).collect::<Vec<_>>();
    let mut in_the_right_order = false;
    let mut iteration_to_keep = false;
    while in_the_right_order == false {
        match check_page_order(pages_numbers.to_owned(), rule_before.clone(), rule_after.clone()) {
            Ok(_pages) => {
                // println!("{:?} is in the right order", pages);
                in_the_right_order = true;
                if iteration_to_keep {
                    // this is the code to get the middle page number
                    // println!("{:?} is in the right order", pages_numbers); 
                    let half = (pages_numbers.len()-1) / 2;
                    // println!("half {}", half);
                    let to_add = pages_numbers[half].parse::<i32>().unwrap();
                    // println!("to_add {}", to_add);
                    sum += to_add;
                }
            }
            Err((pages_numbers_not_ordered, index_to_switch)) => {
                iteration_to_keep = true;
                // println!("{:?} is not the right order because of {:?}", pages_numbers , index_to_switch);
                pages_numbers = order_pages(
                    pages_numbers_not_ordered,
                    index_to_switch);
            }
    }
    }
    });

    println!("sum {}", sum);
}

fn order_pages(pages_numbers: Vec<String>, error_index_to_switch: (usize, usize)) -> Vec<String> {
    let mut swaped_pages = pages_numbers.clone();
    swaped_pages.swap(error_index_to_switch.0, error_index_to_switch.1);
    swaped_pages
}

fn check_page_order(pages_numbers: Vec<String>, rule_before: Vec<&str>, rule_after: Vec<&str>) -> Result<Vec<String>, (Vec<String>, (usize, usize))> {
    let mut in_the_right_order = true;
    let mut index_to_switch: (usize, usize) = (0, 0);
    for i in 0..pages_numbers.len() {
        let target_rule = rule_before
            .iter()
            .enumerate()
            .filter(|f| f.1.eq(&pages_numbers[i]))
            .map(|f| rule_after[f.0])
            .collect::<Vec<_>>();
        target_rule.iter().for_each(|rule| {
            pages_numbers
                .iter()
                .enumerate()
                .filter(|p| p.1.eq(rule))
                .for_each(|p| {
                    if p.0 < i && in_the_right_order{
                        // println!("{} is not in the right order because of {}", page, i);
                        in_the_right_order = false;
                        index_to_switch = (p.0, i);
                    }
                });
        });
    }
    if in_the_right_order {
        Ok(pages_numbers.iter().map(|f| f.to_string()).collect::<Vec<_>>())
    } else {
      Err((pages_numbers.iter().map(|f| f.to_string()).collect::<Vec<_>>(), index_to_switch))  
    }
}

// fn part1(page_order: Vec<&str>, rule_before: Vec<&str>, rule_after: Vec<&str>) {
//     let mut sum = 0;
//     page_order.iter().for_each(|page| {
//         let pages_numbers = page.split(",").collect::<Vec<_>>();
//         let mut in_the_right_order = true;
//         for i in 0..pages_numbers.len() {
//             let target_rule = rule_before
//                 .iter()
//                 .enumerate()
//                 .filter(|f| f.1.eq(&pages_numbers[i]))
//                 .map(|f| rule_after[f.0])
//                 .collect::<Vec<_>>();
//             target_rule.iter().for_each(|rule| {
//                 pages_numbers
//                     .iter()
//                     .enumerate()
//                     .filter(|f| f.1.eq(rule))
//                     .for_each(|f| {
//                         if f.0 < i {
//                             // println!("{} is not in the right order because of {}", page, i);
//                             in_the_right_order = false; 
//                         }
//                     });
//             });
//             if  !in_the_right_order { break; }
//         }
//         if in_the_right_order {
//             // println!("{} is in the right order", page); 
//             let half = (pages_numbers.len()-1) / 2;
//             // println!("half {}", half);
//             let to_add = pages_numbers[half].parse::<i32>().unwrap();
//             // println!("to_add {}", to_add);
//             sum += to_add;
//         }
//     });

//     println!("sum {}", sum);
// }
