use std::ops::Range;

fn num_balanced(to_balance: &String) {
    println!("{}", to_balance);
    let mut open = 0;
    let mut close = 0;
    for c in to_balance.chars() {
        if c == '(' {
            open += 1;
        } else if c == ')' {
            close += 1;
        }
    }
    let to_add=0;
    println!("{}",add);
}

fn main() {
    num_balanced(&String::from("))"));
    println!("Hello, world!");
}
