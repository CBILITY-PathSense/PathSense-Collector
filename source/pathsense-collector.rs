use std::env;
use std::process::Command;

fn run_command(command: &str) {
    let _ = Command::new("chmod").arg("+x").arg(command).status();
    let _ = Command::new("sudo").arg(command).status();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments. Please use install or remove.");
        return;
    }

    match args[1].as_str() {
        "install" => run_command("sh/install.sh"),
        "remove" => run_command("sh/remove.sh"),
        _ => eprintln!("Invalid command. Please use install or remove."),
    }
}
