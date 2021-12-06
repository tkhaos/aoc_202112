fn solve_puzzle() -> i32 {
    let mut horizon: i32 = 0;
    let mut depth: i32 = 0;
    if let Ok(lines) = super::utils::io::read_lines("./testfile/day2_1") {
        for line in lines {
            if let Ok(ip) = line {
                let vec: Vec<&str> = ip.split(" ").collect();
                if vec.len() == 2 {
                    // println!("{}, {}", vec[0], vec[1]);
                    let cmd = vec[0];
                    let step_s = vec[1];
                    let step_i: i32 = step_s.parse::<i32>().unwrap();
                    match cmd {
                        "forward" => horizon += step_i,
                        "down" => depth += step_i,
                        "up" => depth -= step_i,
                        _ => continue,
                    }
                    if depth < 0 {
                        depth = 0;
                    }
                }
            }
        }
    }
    horizon * depth
}

fn solve_puzzle_2() -> i32 {
    let mut horizon: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    if let Ok(lines) = super::utils::io::read_lines("./testfile/day2_2") {
        for line in lines {
            if let Ok(ip) = line {
                let vec: Vec<&str> = ip.split(" ").collect();
                if vec.len() == 2 {
                    // println!("{}, {}", vec[0], vec[1]);
                    let cmd = vec[0];
                    let step_s = vec[1];
                    let step_i: i32 = step_s.parse::<i32>().unwrap();
                    match cmd {
                        "forward" => {
                            horizon += step_i;
                            depth += step_i * aim;
                        }
                        "down" => aim += step_i,
                        "up" => aim -= step_i,
                        _ => continue,
                    }
                    if depth < 0 {
                        depth = 0;
                    }
                    if aim < 0 {
                        aim = 0;
                    }
                }
            }
        }
    }
    horizon * depth
}

pub fn run() {
    println!("The answer of Puzzle 1 is: {}", solve_puzzle());
    println!("The answer of Puzzle 2 is: {}", solve_puzzle_2());
}
