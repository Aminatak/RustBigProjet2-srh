use std::fmt;
/// Init struct to model one Command
#[derive(Clone, Debug)]
pub struct Command {
    pub command_name: String,
    pub command_args: Vec<String>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args: String = String::from("");
        for i in 0..self.command_args.len() {
            args.push_str(&self.command_args[i])
        }
        write!(f, "{}   {}", self.command_name, args)
    }
}

impl Command {
    /// Function make build command.
    pub fn new(name: String, args: Vec<String>) -> Command {
        Command {
            command_name: name,
            command_args: args,
        }
    }

    pub fn run_command(&self) {
        let status = std::process::Command::new(self.command_name.clone())
            .args(&self.command_args)
            .status()
            .expect("failed to execute command");
        if !status.success() {
            println!("failed to execute");
        }
    }
}
