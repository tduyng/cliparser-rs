use cliparser::{Command, OptionArg};

fn main() {
    let command = Command::new("greet")
        .description("Greet one or more people")
        .option(OptionArg::new(
            Some("n"),
            Some("name"),
            "The name of the person to greet",
        ))
        .action(|_, opts| {
            let name = opts
                .get("name")
                .or_else(|| opts.get("n"))
                .cloned()
                .unwrap_or_else(|| "World".to_string());
            println!("Hello, {}!", name);
        });

    let args: Vec<String> = std::env::args().skip(1).collect();
    let _ = command.parse(&args);
}
