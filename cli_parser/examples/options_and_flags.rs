use cliparser::{Command, OptionArg};

fn main() {
    let command = Command::new("serve")
        .description("Start a development server")
        .option(OptionArg::new(Some("p"), Some("port"), "Specify the port").takes_value(true))
        .option(OptionArg::new(Some("h"), Some("host"), "Specify the host").takes_value(true))
        .option(OptionArg::new(None, Some("verbose"), "Enable verbose mode").takes_value(false))
        .action(|_, options| {
            let default_port = "8080".to_string();
            let port = options.get("port").unwrap_or_else(|| &default_port);
            let default_host = "127.0.0.1".to_string();
            let host = options.get("host").unwrap_or(&default_host);
            let verbose = options.contains_key("verbose");

            println!("Starting server at {}:{}", host, port);
            if verbose {
                println!("Verbose mode enabled.");
            }
        });

    let args: Vec<String> = std::env::args().skip(1).collect();
    let _ = command.parse(&args);
}
