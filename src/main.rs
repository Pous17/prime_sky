use std::process::Command;

fn main() {
    Command::new("clear").status().unwrap();
    println!("{:?}", prime(100000));
}

fn prime(max: u32) -> String {
    let mut prime: bool= false;
    let mut prime_vec: Vec<&str> = Vec::<&str>::new();
    let space: &str = " ";
    let dots: &str = ".";

    for n in 1..max {
        for i in 2..n {
            if n % i == 0 {
                prime = false;
                break;
            } else {
                prime = true;
            }
        }
        if prime == true {
            prime_vec.push(dots);
        } else {
            prime_vec.push(space);
        }
    }
    prime_vec.join("")
}