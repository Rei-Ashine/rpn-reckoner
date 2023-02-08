extern crate rpn_reckoner;
use std::io::{ self, Write };


fn main() -> Result<(), std::io::Error> {
    println!("");
    println!("#########################################");
    println!("#### Reverse Polish Notation Reckoner ###");
    println!("#########################################");
    println!("");

    // ASCII art (Font: colossal)
    println!("8888888b.      8888888b.      888b    888");
    println!("888   Y88b     888   Y88b     8888b   888");
    println!("888    888     888    888     88888b  888");
    println!("888   d88P     888   d88P     888Y88b 888");
    println!("8888888PF      8888888P^      888 Y88b888");
    println!("888 T88b       888            888  Y88888");
    println!("888  T88b      888            888   Y8888");
    println!("888   T88b     888            888    Y888");
    println!("");

    println!("#########################################");
    println!("Type: q<Enter> to exit.");
    println!("");

    loop {
        print!("\n🚀 RPN > ");
        io::stdout().flush()?;
        let mut expression = String::new();

        io::stdin().read_line(&mut expression)?;

        if expression.trim() == "q" { break; }

        let solution = rpn_reckoner::eval(expression);

        match solution {
            Err(e) => println!("\n👻 {}", e),
            Ok(r) => println!("\n✅ {}", r),
        }
    }
    Ok(())
}