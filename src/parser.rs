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
        let mut tokens : HashMap<String,String>  = HashMap::new();

        if args.len() < 3 {
            return Err(String::from("Few arguments"));
        }

        if args.len() > 5 {
            return Err(String::from("too many arguments"));
        }

        tokens.insert("method".to_string(),args[1].clone());
        tokens.insert("url".to_string(),args[2].clone());
       
        let method =  match tokens.get("method"){
            Some(m) => m.to_string(),
            None => "".to_string()
        };

        let url =  match tokens.get("url"){
            Some(u) => u.to_string(),
            None => "".to_string()
        };

        

        Ok(Args{method , url })


   }


}

