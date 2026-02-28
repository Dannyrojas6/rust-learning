// use std::io::{self, Write};

// fn main() {
//     let mut input = String::new();

//     loop {
//         print!(">>> ");
//         io::stdout().flush().unwrap();

//         input.clear();

//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         let input = input.trim();

//         let parts: Vec<&str> = input.split_whitespace().collect();

//         if parts.len() == 3 {
//             let left: f64 = parts[0].parse().unwrap();
//             let right: f64 = parts[2].parse().unwrap();
//             let op = parts[1];

//             let result = match op {
//                 "+" => left + right,
//                 "-" => left - right,
//                 "*" => left * right,
//                 "/" => left / right,
//                 _ => {
//                     println!("请输入正确的操作符！");
//                     continue;
//                 }
//             };
//             println!("{}", result);
//         }
//         // if input == "quit" || input == "q" {
//         if matches!(input, "quit" | "q") {
//             println!("exit with 0");
//             break;
//         }
//         println!("Your input: {}", input);
//     }
// }
use std::io::{self, Write};
enum Operator {
    Add,
    Subtract,
    Muiltply,
    Divide,
}
fn parse_expression(input: &str) -> Result<(f64, Operator, f64), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("Format error!please enter : left operator right (e.g. 4 + 5)".to_string());
    }

    let left: f64 = parts[0].parse().map_err(|_| "left input is not num!")?;
    let right: f64 = parts[2].parse().map_err(|_| "right input is not num!")?;

    let operator = match parts[1] {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Muiltply,
        "/" => Operator::Divide,
        _ => return Err("Operator is invalid!".to_string()),
    };

    Ok((left, operator, right))
}
fn calculate(left:f64,op:Operator,right:f64)->Result<>
fn main() {
    let mut input = String::new();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed t read line");
        let input = input.trim();

        if matches!(input, "quit" | "q") {
            println!("exit with 0");
            break;
        }
    }
}
