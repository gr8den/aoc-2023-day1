
fn get_digit(s: &str, mapping: &Vec<(&str, usize)>) -> Option<usize> {
    for (str_digit, int_digit) in mapping {
        if s.starts_with(str_digit) {
            return Some(*int_digit);
        }
    }

    None
}

fn main() {
    let mapping: Vec<(&str, usize)> = vec![
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),

        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut s = 0;
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let mut a = 0;
            let mut b = 0;

            for i in 0..line.len() {
                if let Some(digit) = get_digit(&line[i..], &mapping) {
                    a = digit;
                    break;
                }
            }

            for i in (0..line.len()).rev() {
                if let Some(digit) = get_digit(&line[i..], &mapping) {
                    b = digit;
                    break;
                }
            }

            s += a * 10 + b;
        } else {
            break;
        }
    }
    println!("{s}");
}
