mod parser;
use std::env;

use parser::Command;
use parser::CommandType;



fn main() -> Result<(), Box<dyn std::error::Error>> {
     let mut command_line_args = env::args().collect::<Vec<String>>();
     command_line_args.remove(0);
     let command_to_execute = command_line_args.get(0).unwrap();
      let command = Command::new(CommandType::from_str(command_to_execute).unwrap(), command_line_args.get(1).unwrap().clone());
      let body = command_line_args.get(2).unwrap_or(&String::from("")).clone();
      
      command.execute(&body);



    Ok(())
}
