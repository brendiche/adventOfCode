use std::{fs::read_to_string, io::{stdout, Write}, thread, time};

use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};

const DIRECTIONS: [(&str,(i32,i32),&str); 4] = [("v",(1,0),"<"), ("^", (-1,0), ">"), ("<", (0,-1),"^"), (">", (0,1), "v")];

fn main() {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    let mut map = read_input("./assets/input");
    display_map(map.clone());
    let mut in_bound = true;
    let mut guard_coord = get_guard_coord(map.clone()).unwrap_or_else(|_| panic!("Guard not found"));
    while in_bound {
        // println!("{:?}", guard_coord);
        //  loop  over it
        match guard_move(map.clone(), guard_coord) {
            Some((map_updated, guard_coord_updated)) => {
                display_map(map_updated.clone());
                // println!("");
                map = map_updated;
                guard_coord = guard_coord_updated;
            }
            None  => {
                in_bound = false;
            }
        }
    }
    // count X in matrix
    // display_map(map.clone());
    let mut distinct_positions = 1;
    map.iter().for_each(|row|{
        row.iter().for_each(|c|{
            if c.eq("X") {distinct_positions += 1}
        });
    });

    println!("the guard will visit {} distinct position", distinct_positions);
}


fn read_input(path: &str) -> Vec<Vec<String>> {
    read_to_string(path).unwrap()
        .lines()
        .map(|row| row.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn display_map(map: Vec<Vec<String>>) {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            stdout.write_all(format!("{}", map[i][j]).as_bytes()).unwrap();
        }
        stdout.write_all(format!("\n").as_bytes()).unwrap();
    }
    // thread::sleep(time::Duration::from_millis(1));
    stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    stdout.execute(cursor::Show).unwrap();

}

fn get_guard_coord(map: Vec<Vec<String>>) -> Result<(usize, usize), String> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if DIRECTIONS.iter().map(|d| d.0).collect::<Vec<_>>().contains(&map[i][j].as_str()) { return Ok((i, j)) }
        }
    }

    Err(String::from("Guard not found"))
}

fn  guard_move(map: Vec<Vec<String>>, guard_coord: (usize, usize)) -> Option<(Vec<Vec<String>>, (usize, usize))>  {
    let guard = map[guard_coord.0][guard_coord.1].clone();
    let mut updated_map= map.clone();
    let mut out_bound = false;
    let mut next_i = 0;
    let mut next_j = 0;
    if DIRECTIONS[0].0.eq(guard.as_str()) {
        (updated_map, (next_i, next_j), out_bound) = bite(updated_map, guard_coord, DIRECTIONS[0]).unwrap();
    } else if DIRECTIONS[1].0.eq(guard.as_str()) {
        (updated_map, (next_i, next_j), out_bound) = bite(updated_map, guard_coord, DIRECTIONS[1]).unwrap(); 
    } else if DIRECTIONS[2].0.eq(guard.as_str()) {
        (updated_map, (next_i, next_j), out_bound) = bite(updated_map, guard_coord, DIRECTIONS[2]).unwrap();
    } else if DIRECTIONS[3].0.eq(guard.as_str()) {
        (updated_map, (next_i, next_j), out_bound) = bite(updated_map, guard_coord, DIRECTIONS[3]).unwrap();
    }
    
        DIRECTIONS
        .iter()
        .for_each(|d|{
            if  d.0.eq(guard.as_str()) { 
                // println!("Guard  is going : {}", d.0);
                // check and set next_position
                
            }
        });
    if out_bound{
        None
    } else {

        Some((updated_map, (next_i, next_j)))
    } 
}

fn bite(mut updated_map: Vec<Vec<String>>, guard_coord: (usize, usize), to_be_def: (&str,(i32,i32),&str)) -> Result<(Vec<Vec<String>>, (usize, usize), bool), String> {
    let mut clean_possition = false;
    let mut out_bound = false;
    let mut next_i = guard_coord.0 as i32 + to_be_def.1.0;
    let mut next_j = guard_coord.1 as i32 + to_be_def.1.1;
    // println!("next position will be: ({},{})", next_i,next_j);
    if 
        next_i < 0 || 
        next_i >= updated_map.len() as i32 || 
        next_j < 0 || 
        next_j >= updated_map[guard_coord.0].len() as i32 {
        // println!("should return that guard is outside the  map");
        out_bound = true;
    } else {
        if updated_map[next_i as usize][next_j as usize].eq("#") {
            updated_map[guard_coord.0][guard_coord.1] = to_be_def.2.to_string();
        } else {
            updated_map[next_i as usize][next_j as usize] = to_be_def.0.to_string();
            clean_possition = true;
        }
    }
    // update last position
    if clean_possition {
        updated_map[guard_coord.0][guard_coord.1] = "X".to_string();
    } else {
        next_i = guard_coord.0 as i32;
        next_j = guard_coord.1 as i32;
    }

    Ok((updated_map, (next_i as usize, next_j as usize), out_bound))
}