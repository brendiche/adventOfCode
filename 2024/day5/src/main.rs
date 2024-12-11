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

    // println!("rules_before {:?}", rule_before);
    // println!("rules_after {:?}", rule_after);
    // println!("page order {:?}", page_order);

    let mut sum = 0;
    page_order.iter().for_each(|page| {
        // .map(|n| n.parse::<i32>().unwrap())
        let pages_numbers = page.split(",").collect::<Vec<_>>();
        let mut in_the_right_order = true;
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
                    .filter(|f| f.1.eq(rule))
                    .for_each(|f| {
                        if f.0 < i {
                            // println!("{} is not in the right order because of {}", page, i);
                            in_the_right_order = false; 
                        }
                    });
            });
            if  !in_the_right_order { break; }
        }
        if in_the_right_order {
            // println!("{} is in the right order", page); 
            let half = (pages_numbers.len()-1) / 2;
            // println!("half {}", half);
            let to_add = pages_numbers[half].parse::<i32>().unwrap();
            // println!("to_add {}", to_add);
            sum += to_add;
        }
    });

    println!("sum {}", sum);
}
