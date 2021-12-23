use crate::utils;

fn solve_puzzle() -> u32 {
    if let Ok(lines) = utils::io::read_lines("./testfile/day3_1") {
        let mut cnt: u32 = 0;
        let mut data: Vec<u32> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                if cnt == 0 {
                    data = ip.chars().map(|x| x as u32 - 0x30).collect();
                } else {
                    ip.chars()
                        .enumerate()
                        .for_each(|(i, x)| data[i] += x as u32 - 0x30)
                }
                cnt += 1;
            }
        }
        let mut gamma: u32 = 0;
        let mut eps: u32 = 0;
        for x in data.iter() {
            gamma = gamma << 1;
            eps = eps << 1;
            if x > &(cnt - x) {
                gamma = gamma | 1;
            } else {
                eps = eps | 1;
            }
        }
        return eps * gamma;
    };
    0
}

fn get_rating(mut data: Vec<String>, f: fn(cnt_one: u32, cnt_zero: u32, c: char) -> bool) -> u32 {
    let mut i = 0;
    let length = data[0].len();
    while i < length {
        let mut cnt: u32 = 0;
        let mut tag: u32 = 0;
        for s in data.iter() {
            tag += s.chars().nth(i).unwrap() as u32 - 0x30;
            cnt += 1;
        }
        let mut j = 0;
        loop {
            if j >= data.len() || data.len() == 1 {
                break;
            }
            let c = data[j].chars().nth(i).unwrap();
            if f(tag, cnt - tag, c) {
                data.swap_remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    isize::from_str_radix(data[0].as_str(), 2).unwrap() as u32
}

fn oxy_checker(cnt_one: u32, cnt_zero: u32, c: char) -> bool {
    (cnt_one >= cnt_zero && c == '0') || (cnt_one < cnt_zero && c == '1')
}

fn co2_checker(cnt_one: u32, cnt_zero: u32, c: char) -> bool {
    (cnt_one >= cnt_zero && c == '1') || (cnt_one < cnt_zero && c == '0')
}

fn solve_puzzle_2() -> u32 {
    if let Ok(lines) = utils::io::read_lines("./testfile/day3_2") {
        let mut data: Vec<String> = Vec::new();
        for line in lines {
            data.push(line.unwrap());
        }
        let mut data2 = data.to_vec();
        let oxy = get_rating(data, oxy_checker);
        let co2 = get_rating(data2, co2_checker);
        return oxy * co2;
    };
    0
}

pub fn run() {
    println!("The answer of Puzzle 1 is: {}", solve_puzzle());
    println!("The answer of Puzzle 2 is: {}", solve_puzzle_2());
}
