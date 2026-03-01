use std::io::{self, Write};
#[derive(Debug, Clone, Copy)]

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Calculator {
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }
    fn add_to_history(&mut self, record: String) {
        self.history.push(record);
    }
    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No history yet.");
            return;
        }
        println!("====== history ======");
        for (i, record) in self.history.iter().enumerate() {
            println!("{}. {}", i + 1, record);
        }
    }
    fn clear_history(&mut self) {
        self.history.clear();
    }
    fn into_history(self) -> Vec<String> {
        self.history
    }
}

fn parse_expression(input: &str) -> Result<(f64, Operator, f64), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("format error! (e.g. 2 * 4)".to_string());
    }
    let left: f64 = parts[0]
        .parse()
        .map_err(|_| format!("{} is not valid num!", parts[0]))?;
    let right: f64 = parts[2]
        .parse()
        .map_err(|_| format!("{} is not valid num!", parts[2]))?;
    let operator = match parts[1] {
        "+" => Operator::Add,
        "-" => Operator::Subtract,
        "*" => Operator::Multiply,
        "/" => Operator::Divide,
        _ => {
            return Err("operator invalid!".to_string());
        }
    };
    Ok((left, operator, right))
}

fn calculate(left: f64, operator: Operator, right: f64) -> Result<f64, String> {
    match operator {
        Operator::Add => Ok(left + right),
        Operator::Subtract => Ok(left - right),
        Operator::Multiply => Ok(left * right),
        Operator::Divide => {
            if right != 0.0 {
                Ok(left / right)
            } else {
                Err("divide zero error!".to_string())
            }
        }
    }
}

fn format_result(left: f64, operator: &Operator, right: f64, result: f64) -> String {
    let op = match operator {
        Operator::Add => "+",
        Operator::Subtract => "-",
        Operator::Multiply => "*",
        Operator::Divide => "/",
    };
    format!("{} {} {} = {}", left, op, right, result)
}

fn main() {
    let mut calc = Calculator::new();
    let mut input = String::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input {
            "quit" | "exit" | "q" => {
                println!("exit with 0");
                break;
            }
            "history" => {
                calc.show_history();
                continue;
            }
            "clear" => {
                calc.clear_history();
                continue;
            }
            "" => continue,
            _ => {}
        }

        match parse_expression(input) {
            Ok((left, operator, right)) => match calculate(left, operator, right) {
                Ok(result) => {
                    let record = format_result(left, &operator, right, result);
                    println!("{}", record);
                    calc.add_to_history(record);
                    // println!("{}", result);
                }
                Err(e) => println!("calculate error:{}", e),
            },
            Err(e) => println!("parse error:{}", e),
        }
    }
}
