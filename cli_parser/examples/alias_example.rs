use cliparser::Command;

fn main() {
    let command = Command::new("run")
        .alias("r")
        .description("Run the program")
        .action(|_, _| {
            println!("Running the program...");
        });

    let args: Vec<String> = std::env::args().skip(1).collect();
    let _ = command.parse(&args);
}
