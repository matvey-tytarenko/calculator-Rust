use std::io;

fn main() {
    let mut num_a = String::new();
    let mut num_b = String::new();
    let mut operator = String::new();
    let input = io::stdin();

    println!("Input first number");
    input.read_line(&mut num_a);

    println!("Input second number");
    input.read_line(&mut num_b);

    println!("Input operator");
    input.read_line(&mut operator);

    let num_first: i32 = match num_a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error Parsing number!");
            return;
        }
    };

    let num_second: i32 = match num_b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error Parsing number!");
            return main();
        }
    };

    let operator = operator.trim();

    match operator {
        "+" => println!("{}", num_first + num_second),
        "-" => println!("{}", num_first - num_second),
        "*" => println!("{}", num_first * num_second),
        "=" => println!("{}", num_first = num_second),
        "/" => {
            if num_second == 0 {
                println!("Devide by 0? IT'S IMPOSIBLE!");
            } else {
                println!("{}", num_first / num_second);
            };
        },
        _ => println!("Error!"),
    };

    println!("Press Enter to exit...");
    let mut exit = String::new();
    input.read_line(&mut exit);
}
