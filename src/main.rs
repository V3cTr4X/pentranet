use colored::*; // Importa el trait para usar colores
use std::io;

fn main() {
    // Imprimir el logo en ASCII en rojo
    println!("{}", "▗▄▄▖ ▗▞▀▚▖▄▄▄▄     ■   ▄▄▄ ▗▞▀▜▌▄▄▄▄  ▗▞▀▚▖   ■".red());
    println!("{}", "▐▌ ▐▌▐▛▀▀▘█   █ ▗▄▟▙▄▖█    ▝▚▄▟▌█   █ ▐▛▀▀▘▗▄▟▙▄▖".red());
    println!("{}", "▐▛▀▘ ▝▚▄▄▖█   █   ▐▌  █         █   █ ▝▚▄▄▖  ▐▌".red());
    println!("{}", "▐▌                ▐▌                         ▐▌".red());
    println!("{}", "                  ▐▌                         ▐▌".red());
    println!("{}", "                                                 ".red());
    println!("{}", "               Coded by V3cTr4x                    ".red());

    println!("----------------------------");
    println!("Option menu:");
    println!("1. Network Attack");
    println!("2. Exploit searcher");
    println!("3. Exit");        
    println!("----------------------------");
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option: i32 = option.trim().parse().unwrap(); 

 
    match option {
        1 => network_attack(),
        2 => exploit_searcher(),
        3 => exit_program(),
        _ => println!("Invalid option! Please choose 1, 2, or 3."),
    }
}

fn network_attack() {
    println!("{}", "Network Attack selected!".green());
    
}

fn exploit_searcher() {
    println!("{}", "Exploit searcher selected!".green());
    
}

fn exit_program() {
    println!("{}", "Exiting program...".yellow());
    
}


