use super::super::utils;

fn solve_puzzle(_magic_num: usize) -> i32 {
    let mut ret: i32 = 0;
    if let Ok(lines) = utils::io::read_lines("./testfile/hosts") {
        // Consumes the iterator, returns an (Optional) String
        let mut depths = vec![0; _magic_num];
        // let mut depths: [i32; _magic_num] = [0; _magic_num];
        let mut index: usize = 0;
        let mut increase_num: i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let depth: i32 = ip.parse::<i32>().unwrap();
                depths[(index % _magic_num)] = depth;
                if index >= (_magic_num - 1) {
                    if depths[index % _magic_num] > depths[(index - (_magic_num - 1)) % _magic_num]
                    {
                        increase_num += 1;
                    }
                }
                index += 1;
            }
        }
        ret = increase_num;
    }
    ret
}

pub fn run() {
    println!("The answer of Puzzle 1 is: {}", solve_puzzle(2));
    println!("The answer of Puzzle 2 is: {}", solve_puzzle(4));
}
