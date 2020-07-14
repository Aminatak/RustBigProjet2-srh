mod shell;
use shell::RshJobs;
use shell::Job;
use shell::parser::commands::Command;
use shell::parser::parse_command;
use std::io;

fn main()  -> std::io::Result<()> {
    let mut rsh = RshJobs::new();
    let job1 = Job{ job_id : 1, job_command : String::from("micro-shell")};
    let res = rsh.add_job(job1);
    let stdout = io::stdout();
    let stdin = io::stdin();


    loop{
        let _res = rsh.command_prompt(&stdout);
        let mut user_input = String::with_capacity(256);
        let _cmd = rsh.wait_command_line(&mut user_input, &stdin);
        user_input = user_input.trim_end().trim_end().to_string();
        // Enter
        if user_input.len() == 0 {
            continue;
        }
        // Quit terminal rsh => stop programe
        if user_input == "quit" {
            break;
        }
        // 
        let cmd : Command = parse_command(&mut user_input);
        
        if cmd.command_name == String::from("zaadps"){
            let _res = rsh.zaad_ps(&stdout);
            continue
        }
        cmd.run_command()
    }
    res
}