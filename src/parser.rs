use std::env;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Args{
    pub method : String,
    pub url : String,
   
}

// implementation
impl Args{

    pub fn new() -> Result<Args,String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err(String::from("Few arguments"));
        }

        /*if args.len() > 6 {
            return Err(String::from("too many arguments"));
        }*/
        let method = args[1].clone();
        let url = args[2].clone();

        Ok(Args{method , url })


   }


}

