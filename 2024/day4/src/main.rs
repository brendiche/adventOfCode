
fn main() {
    let lines = std::fs::read_to_string("./assets/input_test").unwrap();
    let  matrix = lines.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("{}", matrix[i][j]
            // look for char X
        
        )
        }
        println!();
    }
}