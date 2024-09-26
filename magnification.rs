use std::io::{stdin,stdout,Write};

fn input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return s;
}

fn main() {
    print!("ho: ");
    let ho = input().parse::<f32>().unwrap();

    print!("hi: ");
    let mut hi = input().parse::<f32>().unwrap();

    print!("upright (u) / virtual (v): ");
    let attitude = input().to_lowercase();

    if attitude == "virtual" || attitude == "v" {
        hi *= -1.0;
    }

    println!("\nanswer:\nM: {}", hi/ho);
}
