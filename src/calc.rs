use std::io::{Write};
use std::fs::OpenOptions;
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
unsafe extern "C" {
    fn sum(a: f64,b: f64) -> f64;
    fn subtract(a: f64,b: f64) -> f64;
    fn multiply(a: f64,b: f64) -> f64;
    fn divide(a: f64,b: f64,result: *mut divide_result) -> bool;
    fn volume_cylinder(pi: f64,r: f64,h: f64) -> f64;
    fn vertical(a: f64,b: f64,c: f64,result: *mut vertical_result) -> bool;
    fn bhaskara(a: f64,b: f64,c: f64,result: *mut bhaskara_result) -> bool;
    fn pitagoras(hipotenuse: f64,cateto_oposto: f64,cateto_adjacente: f64) -> f64;
}
#[repr(C)]
struct bhaskara_result {
    x1: f64,
    x2: f64,
}
#[repr(C)]
struct divide_result {
    ab: f64,
}
#[repr(C)]
struct vertical_result {
    x: f64,
    y: f64,
}
struct RemoveHistory;
struct Read;
struct Save;
struct Myinput;
struct Calc;
struct Bhaskara;
struct Vertical;
struct VolumeCylinder;
struct Pitagoras;
pub struct Calculator;
pub trait Calculators {
    fn calculator() -> std::io::Result<bool>;
}
trait Pitagorass {
    fn values() -> std::io::Result<(f64,f64,f64)>;
    fn pitagoras_for_rs() -> std::io::Result<()>;
    fn principal_pitagoras() -> std::io::Result<()>;
}
trait RemoveHistorys {
    fn remove_history(file: &str) -> std::io::Result<()>;
}
trait Reads {
    fn read(file: &str) -> std::io::Result<()>;
}
trait Saves {
    fn save(something: &str,filename: &str) -> std::io::Result<()>;
}
trait Myinputs {
    fn output(msg: &str) -> std::io::Result<f64>;
    fn output_text(msg: &str) -> std::io::Result<String>;
}
trait Calcs {
    fn my_nums() -> std::io::Result<(f64,f64)>;
    fn calc() -> std::io::Result<()>;
}
trait Bhaskaras {
    fn my_abc() -> std::io::Result<(f64,f64,f64)>;
    fn bhaskara_for_rs() -> std::io::Result<()>;
    fn principal_bhaskara() -> std::io::Result<()>;
}
trait Verticals {
    fn values() -> std::io::Result<(f64,f64,f64)>;
    fn vertical_for_rs() -> std::io::Result<()>;
    fn principal_vertical() -> std::io::Result<()>;
}
trait VolumeCylinders {
    fn pi_r_h() -> std::io::Result<(f64,f64,f64)>;
    fn volume_cylinder_for_rs() -> std::io::Result<()>;
    fn principal_volume_cylinder() -> std::io::Result<()>;
}
impl RemoveHistorys for RemoveHistory {
    fn remove_history(file: &str) -> std::io::Result<()> {
        std::fs::write(&file,"")?;
        return Ok(());
    }
}
impl Reads for Read {
    fn read(file: &str) -> std::io::Result<()> {
        let text = std::fs::read_to_string(&file)?;
        if text.is_empty() {
            println!("Empty history, no calculations done");
            return Ok(());
        }
        else {
            println!("\n{}\n",text);
            return Ok(());
        }
    }
}
impl Saves for Save {
    fn save(something: &str,filename: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&filename)?;
        let save = format!("{}",something);
        writeln!(file,"{}",save)?;
        return Ok(());
    }
}
impl Myinputs for Myinput {
    fn output(msg: &str) -> std::io::Result<f64> {
        println!("\n{msg}");
        loop {
            print!("{BLUE}~{RESET} V ");
            std::io::stdout().flush()?;
            let mut num = String::new();
            std::io::stdin().read_line(&mut num)?;
            match num.trim().parse::<f64>() {
                Ok(n) => return Ok(n),
                Err(_) => println!("{BOLD}{RED}ERR101{RESET}{BOLD}: Type only Numbers"),
            }
        }
    }
    fn output_text(msg: &str) -> std::io::Result<String> {
        println!("\n{msg}");
        print!("{BLUE}~{RESET} V ");
        std::io::stdout().flush()?;
        let mut text = String::new();
        std::io::stdin().read_line(&mut text)?;
        let text = text.trim().to_string();
        return Ok(text);
    }
}
impl Pitagorass for Pitagoras {
    fn values() -> std::io::Result<(f64,f64,f64)> {
        let h = Myinput::output("Enter the value of hipotenuse")?;
        let cp = Myinput::output("Enter the value of the opposite side")?;
        let ca = Myinput::output("Enter the value of the adjacent side")?;
        return Ok((h,cp,ca));
    }
    fn pitagoras_for_rs() -> std::io::Result<()> {
        let (h,cp,ca) = Pitagoras::values()?;
        let result = unsafe {pitagoras(h,cp,ca)};
        println!("\n{}\n",result);
        if h == 1.0 && cp == 1.0 && ca == 1.0 {
            println!("{BOLD}{RED}ERR404{RESET}{BOLD}: All values are equal to 1, this is not valid in traditional plane geometry{RESET}");
            return Ok(());
        }
        else if h == 1.0 && cp > 1.0 && ca > 1.0 {
        let default_save = format!(r#"

              |\
              | \ x
          {ca}|  \
              |___\
              {cp}

            "#);
            Save::save(&default_save,"pitagoras.txt")?;
            return Ok(());
        }
        else if h > 1.0 && cp == 1.0 && ca > 1.0 {
            let save = format!(r#"

                |\
                | \ {h}
           {ca} |  \
                |___\
                  x

                  "#);
            Save::save(&save,"pitagoras.txt")?;
            return Ok(());
        }
        else if h > 1.0 && cp > 1.0 && ca == 1.0 {
            let save = format!(r#"

             |\
             | \ {h}
          x  |  \
             |___\
              {cp}

              "#);
              Save::save(&save,"pitagoras.txt")?;
              return Ok(());
        }
        else if h > 1.0 && cp > 1.0 && ca > 1.0 {
            let save = format!(r#"

                |\
                | \ {h}
           {ca} |  \
                |___\
                {cp}

                "#);
                Save::save(&save,"pitagoras.txt")?;
                return Ok(());
        }
        return Ok(());
    }
    fn principal_pitagoras() -> std::io::Result<()> {
        println!("\n{BOLD}{BLUE}[Pitagoras function]{RESET}\n");
        Pitagoras::pitagoras_for_rs()?;
        loop {
            println!("[1] Try again");
            println!("[2] History");
            println!("[3] Remove history");
            println!("[4] Quit");
            let option = Myinput::output_text("Choose a option")?;
            match option.trim() {
                "1" => Pitagoras::pitagoras_for_rs()?,
                "2" => Read::read("pitagoras.txt")?,
                "3" => {
                    println!("Are you sure do you want to remove the history? [y/n]");
                    let option = Myinput::output_text("Choose a option")?;
                    match option.trim() {
                        "y" | "yes" => RemoveHistory::remove_history("pitagoras.txt")?,
                        "n" | "no" => continue,
                        _=> {println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}"); continue;}
                    }
                }
                "4" => return Ok(()),
                _=> {println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}\n"); continue;}
            }
        }
    }
}
impl Calcs for Calc {
    fn my_nums() -> std::io::Result<(f64,f64)> {
        let a = Myinput::output("type a number")?;
        let b = Myinput::output("type another number")?;
        return Ok((a,b));
    }
    fn calc() -> std::io::Result<()> {  
        println!("{BOLD}{BLUE}[Calc operations]{RESET}");
        loop {
        println!("[1] Sum (+)");
        println!("[2] Subtract (-)");
        println!("[3] Multiply (*)");
        println!("[4] Divide (/)");
        println!("[5] History");
        println!("[6] Remove History");
        println!("[7] Quit");
        let option = Myinput::output_text("Choose a option")?;
        if option.is_empty() {continue;}
        else {
            match option.trim() {
            "1" => {
                let (a,b) = Calc::my_nums()?;
                let result = unsafe {sum(a,b)};
                let save = format!("{} + {} = {}",a,b,result);
                Save::save(&save,"operations.txt")?;
                println!("\n{}\n",result);
            }
            "2" => {
                let (a,b) = Calc::my_nums()?;
                let result = unsafe {subtract(a,b)};
                let save = format!("{} - {} = {}",a,b,result);
                Save::save(&save,"operations.txt")?;
                println!("\n{}\n",result);
            }
            "3" => {
                let (a,b) = Calc::my_nums()?;
                let result = unsafe {multiply(a,b)};
                let save = format!("{} * {} = {}",a,b,result);
                Save::save(&save,"operations.txt")?;
                println!("\n{}\n",result);
            }
            "4" => {
                let (a,b) = Calc::my_nums()?;
                let mut result = divide_result {
                    ab: 0.0,
                };
                let success = unsafe {divide(a,b,&mut result)};
                if success {
                    println!("\n{}\n",result.ab);
                    let save = format!("{} / {} = {}",a,b,result.ab);
                    Save::save(&save,"operations.txt")?;
                }
                else {
                    println!("\n{BOLD}Cannot divide by 0{RESET}\n");
                }
            }
            "5" => Read::read("operations.txt")?,
            "6" => {
                println!("Are you sure you want to delete your history? [y/n]");
                let option = Myinput::output_text("Choose a option")?;
                match option.trim() {
                    "y" | "yes" => RemoveHistory::remove_history("operations.txt")?,
                    "n" | "no" => continue,
                    _=> continue,
                }
            }
            "7" => return Ok(()),
            _=> {
                println!("\n{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}\n");
            }
        }
        }
    }
}
}
impl Bhaskaras for Bhaskara {
    fn my_abc() -> std::io::Result<(f64,f64,f64)> {
        let a = Myinput::output("enter the value of A")?;
        let b = Myinput::output("enter the value of B")?;
        let c = Myinput::output("enter the value of C")?;
        return Ok((a,b,c));
    }
    fn bhaskara_for_rs() -> std::io::Result<()> {
        let (a,b,c) = Bhaskara::my_abc()?;
        if a.abs() < 1e-10 {
            println!("{BOLD}For it to be a quadratic equation, A needs to be greater than 0{RESET}\n");
            return Ok(());
        }
        else {
            let mut result = bhaskara_result {
                x1: 0.0,
                x2: 0.0,
            };
            let success = unsafe {bhaskara(a,b,c,&mut result)};
            if success {
                println!("\n({},{})\n",result.x1,result.x2);
                let save = format!(r#"
                
                {}x² {}x {} = 0
                |
                |
                ({},{})

                "#,a,b,c,result.x1,result.x2);
                Save::save(&save,"bhaskaras.txt")?;
                return Ok(());
            }
            else {
                println!("This Equation don't have real roots");
                println!("\nTry again? [y/n] or history? [h]");
                let save = format!(r#"

                {}x² {}x {} = 0
                |
                |
                This Equation don't have real roots

                "#,a,b,c);
                Save::save(&save,"bhaskaras.txt")?;
                return Ok(());
            }
        }
    }
    fn principal_bhaskara() -> std::io::Result<()> {
        println!("{BOLD}{BLUE}[Bhaskara Function]{RESET}");
        Bhaskara::bhaskara_for_rs()?;
        loop {
            println!("\n[1] Try again");
            println!("[2] History");
            println!("[3] Remove history");
            println!("[4] Quit");
            let option = Myinput::output_text("Choose a option")?;
            if option.is_empty() {continue;}
            match option.trim() {
                "1" => continue,
                "2" => Read::read("bhaskaras.txt")?,
                "3" => {
                    println!("Are you sure do you want to remove the history? [y/n]");
                    let option = Myinput::output_text("Choose a option")?;
                    match option.trim() {
                        "y" | "yes" => RemoveHistory::remove_history("bhaskaras.txt")?,
                        "n" | "no" => continue,
                        _=> {println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}"); continue;}
                    }
                }
                "4" => return Ok(()),
                _=> {println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}"); continue;}
            }
        }
    }
}
impl Verticals for Vertical {
    fn values() -> std::io::Result<(f64,f64,f64)> {
        let a = Myinput::output("Enter the value of A")?;
        let b = Myinput::output("Enter the value of B")?;
        let c = Myinput::output("Enter the value of C")?;
        return Ok((a,b,c));
    }
    fn vertical_for_rs() -> std::io::Result<()> {
        let (a,b,c) = Vertical::values()?;
        if a.abs() < 1e-10 {
            println!("{BOLD}For it to be a quadratic equation, A needs to be greater than 0{RESET}\n");
            return Ok(());
        }
        else {
            let mut result = vertical_result {
                x: 0.0,
                y: 0.0,
            };
            let success = unsafe {vertical(a,b,c,&mut result)};
            if success {
                println!("\n({},{})\n",result.x,result.y);
                let save = format!(r#"

                {}x² {}x {} = 0

                "#,a,b,c);
                Save::save(&save,"verticals.txt")?;
                return Ok(());
            }
            return Ok(());
        }
    }
    fn principal_vertical() -> std::io::Result<()> {
        println!("\n{BOLD}{BLUE}[Vertical Function]{RESET}\n");
        Vertical::vertical_for_rs()?;
        loop {
            println!("[1] Try again");
            println!("[2] History");
            println!("[3] Remove history");
            println!("[4] Quit");
            let option = Myinput::output_text("Choose a option")?;
            if option.is_empty() {continue;}
            match option.trim() {
                "1" => {Vertical::vertical_for_rs()?; continue;}
                "2" => {Read::read("verticals.txt")?; continue;}
                "3" => {
                    println!("\nAre you sure do you want to remove the history? [y/n]");
                    let option = Myinput::output_text("Choose a option")?;
                    match option.trim() {
                        "y" | "yes" => RemoveHistory::remove_history("verticals.txt")?,
                        "n" | "no" => continue,
                        _=> {println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}"); continue;}
                    }
                }
                "4" => return Ok(()),
                _=> {println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}"); continue;}
            }
        }
    }
}
impl VolumeCylinders for VolumeCylinder {
    fn pi_r_h() -> std::io::Result<(f64,f64,f64)> {
        let pi = Myinput::output("Enter the value of π")?;
        let r = Myinput::output("Enter the value of R")?;
        let h = Myinput::output("Enter the value of H")?;
        return Ok((pi,r,h));
    }
    fn volume_cylinder_for_rs() -> std::io::Result<()> {
        let (pi,r,h) = VolumeCylinder::pi_r_h()?;
        let result = unsafe {volume_cylinder(pi,r,h)};
        println!("\n{}\n",result);
        let save = format!(r#"

        ____________
        |   |      |
        |   |      |
        |   |      |  
        |   |      | {}
        |   |      | 
        |   |_{}___|
        |__________|

        V = {} * {}² * {} = {}

        "#,h,r,pi,r,h,result);
        Save::save(&save,"cylinders.txt")?;
        return Ok(());
    }
    fn principal_volume_cylinder() -> std::io::Result<()> {
        println!("\n{BOLD}{BLUE}[Volume Cylinder Function]{RESET}\n");
        VolumeCylinder::volume_cylinder_for_rs()?;
        loop {
            println!("[1] Try again");
            println!("[2] History");
            println!("[3] Remove History");
            println!("[4] Quit");
            let option = Myinput::output_text("Choose a option")?;
            if option.is_empty() {continue;}
            match option.trim() {
                "1" => VolumeCylinder::volume_cylinder_for_rs()?,
                "2" => Read::read("cylinders.txt")?,
                "3" => {
                    println!("\nAre you sure do you want to remove the history? [y/n]");
                    let option = Myinput::output_text("Choose a option")?;
                    match option.trim() {
                        "y" | "yes" => RemoveHistory::remove_history("cylinders.txt")?,
                        "n" | "no" => continue,
                        _=> continue,
                    }
                }
                "4" => return Ok(()),
                _=> {println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}\n"); continue;}
            }
        }
    }
}
impl Calculators for Calculator {
    fn calculator() -> std::io::Result<bool> {
        println!("\n{BOLD}{BLUE}[Default Function]{RESET}\n");
        println!("[1] Operations [+,-,*,/]");
        println!("[2] quadratic equations");
        println!("[3] Volume of Cylinder");
        println!("[4] Vertical");
        println!("[5] Pitagoras");
        println!("[6] Quit");
        let option = Myinput::output_text("Choose a option")?;
        if option.is_empty() {}
        else {
        match option.trim() {
            "1" => Calc::calc()?,
            "2" => Bhaskara::principal_bhaskara()?,
            "3" => VolumeCylinder::principal_volume_cylinder()?,
            "4" => Vertical::principal_vertical()?,
            "5" => Pitagoras::principal_pitagoras()?,
            "6" => return Ok(false),
            _=> println!("{BOLD}{RED}ERR404{RESET}{BOLD}: Invalid option{RESET}\n"),
        }
    }
        return Ok(true);
}
}
