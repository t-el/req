mod parser;
use std::env;

use parser::Args;
use req::MyClient;


fn main() -> Result<(), Box<dyn std::error::Error>> {
     let client = MyClient::new();
     let parser = Args::new()?;

     match &parser.method.as_str() {
         get => {
             let mut res = client.get_req(&parser.url)?;
             println!("----------------------------------------------");
             for (key, value) in res.headers().iter() {
                println!("{:?}: {:?}", key, value);
             }
             println!("----------------------------------------------");
         
            
             res.copy_to(&mut std::io::stdout())?;
        },
         post=> {
             let args: Vec<String> = env::args().collect();
             let json  = args[3].as_str();
             let mut res = client.post_req(&parser.url, &json)?;
             for (key, value) in res.headers().iter() {
                println!("{:?}: {:?}", key, value);
             }
             println!("----------------------------------------------");
             res.copy_to(&mut std::io::stdout())?;

        },
     };
     

    Ok(())
}
