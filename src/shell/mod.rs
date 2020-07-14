pub mod parser;
use parser::commands::Command;
use std::fmt;
use std::io::{Stdout,Stdin,self,Result,Write};
use std::env;

/// struct Job to model one processus
#[derive(Clone, Debug)]
pub struct Job{
    pub job_id : u32,
    pub job_command : String
}

impl Job{
    /// Function make build job.
    pub fn new(cmd : Command) -> Job {
        let pid = rand::random::<u32>();
        Job{ job_id : pid, job_command : cmd.command_name}
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}   {}", self.job_id, self.job_command)
    }
} 

/// Struct RshJobs to model all processus of our shell
#[derive(Clone, Debug)]
pub struct RshJobs{
    pub jobs : Vec<Job>
}

impl RshJobs {
    /// Function make build shell.
    pub fn new() -> RshJobs {
        let vector:Vec<Job> = Vec::new();
        RshJobs{ jobs : vector}
    }

    /// Function make build job.
    pub fn add_job (&mut self, job : Job) -> Result<()> {
        self.jobs.push(job);
        Ok(())
    }

    /// Delete a process when it is done.
    pub fn remove_job(&mut self, pid:u32) -> Result<()> {
        for i in 0..self.jobs.len() {
            if self.jobs[i].job_id == pid {
                self.jobs.remove(i);
                break;
            }
        }
        Ok(())
    }

    /// Function to print command prompt.
    pub fn command_prompt(&self, stdout:&Stdout) ->  std::io::Result<()> {
        let path = env::current_dir()?;
        let path_as_string : String = path.into_os_string().into_string().unwrap(); // ici j'arrive pas a propager l'erreur avec ?
        let mut handle = stdout.lock();
        let prompt = format!("zaad_shell:~{} $ ", path_as_string);
        let path_as_byte = prompt.into_bytes();
        handle.write_all(&path_as_byte)?;
        handle.flush()?;
        Ok(())
    }

    /// Function to read command line.
    pub fn wait_command_line (&self, user_input: &mut String, stdin: &Stdin) ->std::io::Result<()> {
        stdin.read_line(user_input)?;
        Ok(())
    }

    pub fn zaad_ps(&self, stdout:&Stdout) ->  std::io::Result<()> {
        let mut handle = stdout.lock();
        handle.write_all(b"  PID  CMD\n")?;
        for i in 0..(self.jobs.len()){
            let prompt = format!("{} {}\n", self.jobs[i].job_id , self.jobs[i].job_command);
            let path_as_byte = prompt.into_bytes();
            handle.write_all(&path_as_byte)?;
        };   
        handle.flush()?;
        
        Ok(())
    }
}

impl fmt::Display for RshJobs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for index in 0 .. (self.jobs.len() - 1) {
            println!("{}",self.jobs[index])
        };
        write!(f, "")
    }
} 

