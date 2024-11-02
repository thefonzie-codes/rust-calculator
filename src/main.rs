use std::io;

struct Calculator {
    num1: f64,
    num2: f64,
    operator: char,
}

impl Calculator {
    fn calculate(&self) -> Result<f64, &'static str> {
        match self.operator {
            '+' => Ok(self.num1 + self.num2),
            '-' => Ok(self.num1 - self.num2),
            '*' => Ok(self.num1 * self.num2),
            '/' => {
                if self.num2 == 0.0 {
                    Err("Cannot divide by zero")
                } else {
                    Ok(self.num1 / self.num2)
                }
            }
            _ => Err("Invalid operator"),
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();

    println!("Enter an operator (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operator: char = input.trim().chars().next().expect("Please enter a valid operator");

    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num2: f64 = input.trim().parse().expect("Please input a valid number");

    let calc = Calculator { num1, num2, operator };

    match calc.calculate() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
