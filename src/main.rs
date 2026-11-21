mod calc;
use calc::{Calculator,Calculators};
fn main() -> std::io::Result<()> {
    println!(r#"

  ____                  _ _ _   
 |  _ \ _   _ ___ _____(_) | |_ 
 | |_) | | | / __|_  / | | __|
 |  _ <| |_| \__ \/ /| | | |_ 
 |_| \_\\__,_|___/___|_|_|\__|

 Ruszit 0001- Calculator
 "#);
    loop {if !Calculator::calculator()? {break;}}Ok(())}
