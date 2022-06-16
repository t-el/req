mod parser;
mod request;
use request::CmdRequestBuilder;
use parser::Args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let args = Args::parse_cmd();
   let mut request = CmdRequestBuilder::new(args);


   println!("{:#?}", request);

    Ok(())
}
