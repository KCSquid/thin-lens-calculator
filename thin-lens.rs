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
    const YELLOW: &str = "\x1b[33m";
    const GREEN: &str = "\x1b[32m";
    const BLUE: &str = "\x1b[34m";
    const MAGENTA: &str = "\x1b[35m";
    const ENDL: &str = "\x1b[0m";

    // get variables from user
    println!("{}INPUT:{}", BLUE, GREEN);
    print!("focal point (f): ");
    let f = input().parse::<f32>().unwrap();

    print!("distance from image (di): ");
    let di = input();

    print!("distance from object (do): ");
    let r#do = input();
    print!("{}", ENDL);

    // init
    let known: f32;
    let magnification: f32;
    let known_name: &str;
    let unknown_name: &str;

    // set di/do and names
    if di == "" {
        known = r#do.parse::<f32>().unwrap();
        known_name = "do";
        unknown_name = "di";
    } else {
        known = di.parse::<f32>().unwrap();
        known_name = "di";
        unknown_name = "do";
    }

    // calculate thin-lens answer
    let answer = 1.0/((1.0/f) - (1.0/known));
    
    // calculate magnification
    let ho: f32;
    let hi: f32;
    if di == "" {
        hi = answer;
        ho = known;
    } else {
        ho = answer;
        hi = known;
    }

    magnification = (hi/ho) * -1.0;

    // output variables
    println!("\n{}VARIABLES:{}", BLUE, ENDL);
    println!("{}f: {}\n{}: {}\n{}: {}\nmagnification: {}{}", MAGENTA, 1.0/f, known_name, 1.0/known, unknown_name, answer, magnification, ENDL);

    println!("\n{}FORMULA (THIN-LENS & MAGNIFICATION):{}", BLUE, ENDL);
    println!("{}1/{} = 1/{} - 1/{} = {} - {} = {}", YELLOW, unknown_name, f, known, 1.0/f, 1.0/known, answer);
    println!("M = {}/{} * -1 = {}{}", hi, ho, magnification, ENDL)
}
