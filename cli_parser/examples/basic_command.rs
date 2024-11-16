use cliparser::Command;

fn main() {
    let command = Command::new("hello")
        .description("A simple command to greet the world")
        .action(|_, _| {
            println!("Hello, World!");
        });

    let args: Vec<String> = std::env::args().skip(1).collect();
    let _ = command.parse(&args);
}
