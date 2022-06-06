mod parser;
use parser::Args;
use req::MyClient;


fn main() -> Result<(), Box<dyn std::error::Error>> {
     let client = MyClient::new();
     let parser = Args::new()?;

     match &parser.method.as_str() {
         get => {
             let mut res = client.get_req(&parser.url)?;
             res.copy_to(&mut std::io::stdout())?;
        },
         post=> {
             let mut res = client.post_req(&parser.url)?;
             res.copy_to(&mut std::io::stdout())?;

        },
     };
     

    Ok(())
}
