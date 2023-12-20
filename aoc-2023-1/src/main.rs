fn main() {
    let mut s = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let a = line.chars().find(|&c| c.is_ascii_digit()).unwrap() as u8 - b'0';
            let b = line.chars().rev().find(|&c| c.is_ascii_digit()).unwrap() as u8 - b'0';
            let (a, b) = (a as usize, b as usize);
            s += a * 10 + b;
        } else {
            break;
        }
    }
    println!("{s}");
}
