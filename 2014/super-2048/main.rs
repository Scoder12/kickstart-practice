use std::io;

// Based on https://stackoverflow.com/a/57200055/9196137
macro_rules! parse {
    ($in:expr => $out:ident as $type:ty) => {
        let $out = $in.trim().parse::<$type>().expect("Unable to parse as number");
    };
}

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        read_str!(inner);
        parse!(inner => $out as $type);
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("Unable to read string");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}


fn solve(n: u32, desc: &str, board: Vec<Vec<u32>>) {
    let (xdir, ydir) = match desc {
        "up" => (0, 1),
        "down" => (0, -1),
        "left" => (-1, 0),
        "right" => (1, 0),
        _ => unreachable!()
    };

    
}


fn main() {
    read!(testcases as u32);
    for t in 1..testcases + 1 {
        read_str!(desc);
        let words = desc.trim().split_whitespace().collect::<Vec<&str>>();
        parse!(words[0] => n as u32);
        let desc = words[1];

        let mut board: Vec<Vec<u32>> = Vec::new();
        for _ in 0..n {
            read_vec!(row as u32);
            board.push(row);
        }

        println!("Case #{}:", n);
        solve(n, desc, board);
    }
}
