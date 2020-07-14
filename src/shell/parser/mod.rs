pub mod commands;
use commands::Command;

pub fn parse_command(cmd : &mut String) -> Command{
    let line = cmd.split_whitespace();
    let mut tokens : Vec<String> = Vec::new();
    for s in line {
        tokens.push(s.to_string());
    }
    let command_name : String = tokens[0].clone();
    let mut command_args : Vec<String> = Vec::new();
    for i in 1..tokens.len() {           
        command_args.push(tokens[i].clone());
    }
    Command::new(command_name, command_args)
}

pub fn parse_piped_command_line(command_line : &mut String) -> Vec<Command>{
    let line_splited = command_line.trim().split("|");
    let mut cmds_string : Vec<String> = Vec::new();
    for s in line_splited {
        cmds_string.push(s.to_string());
    }

    let mut cmds_vector : Vec<Command> = Vec::new();    
    for i in 0..cmds_string.len() {           
        cmds_vector.push(parse_command(&mut cmds_string[i].clone()));
    }
    cmds_vector
}
