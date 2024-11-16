use cliparser::Command;

fn main() {
    let add_cmd = Command::new("add")
        .description("Add a new task")
        .action(|args, _| {
            if let Some(task) = args.get(0) {
                println!("Added task: {}", task);
            } else {
                println!("Please specify a task to add.");
            }
        });

    let list_cmd = Command::new("list")
        .description("List all tasks")
        .action(|_, _| {
            println!("Listing all tasks...");
        });

    let root_cmd = Command::new("tasks")
        .description("Task management CLI")
        .subcommand(add_cmd)
        .subcommand(list_cmd);

    let args: Vec<String> = std::env::args().skip(1).collect();
    let _ = root_cmd.parse(&args);
}
