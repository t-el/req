use clap::Parser;
use std::collections::HashMap;


/// Rest API tester just in your terminal. happy testing! :)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Method to use for the request (get, post, put, delete) (default: get)
    #[clap(short='M', long , default_value = "get")]
    pub method: String,

    /// url to use for the request (required) (e.g. http://www.google.com)
    #[clap(short ='U', long)]
    pub url: String,

     /// query string to send to the server (optional) , example : req --method get --url http://localhost:8080/  --query '?name=John'
     #[clap(short ='Q', long ,multiple = true)]
     pub query: Option<Vec<String>>,
     

    /// body to send to the server (for POST and PUT) (optional) (multiple) , example : req --method post --url http://localhost:8080/  --body '{"name":"John"}' 
    #[clap(short='B', long ,multiple = true)]
    pub body: Option<Vec<String>>,


    /// headers to send to the server (optional)(multiple) , example : req --method post --url http://localhost:8080/  --headers '{"Content-Type":"application/json"}'
    #[clap(short = 'H', long ,multiple = true)]
    pub headers: Option<Vec<String>>,

    /// include the headers in the output (default: false)
    #[clap(long="IH")]
    pub include_headers: bool,

    /// include the body in the output (default: false) , (if the body is too large, it will be truncated)
    #[clap(long ="IB" )]
    pub include_body: bool,

    /// include the status code and status message  in the output (default: true)
    #[clap(long="IS", default_value = "0")]
    pub status: i8,

}

impl Args {
    pub fn parse_cmd() -> Self {
        Args::parse()
    }

    pub fn get_method(&self) -> &str {
        &self.method
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_query(&self) -> Option<&Vec<String>> {
        self.query.as_ref()
    }

    pub fn get_body(&self) -> Option<&Vec<String>> {
        self.body.as_ref()
    }

    pub fn get_headers(&self) -> Option<&Vec<String>> {
        self.headers.as_ref()
    }
  
    
      
    
   
    
        
    pub fn include_headers(&self) -> bool {
        self.include_headers == true
    }

    pub fn include_body(&self) -> bool {
        self.include_body == false
    }

    pub fn include_status(&self) -> bool {
        self.status == 1
    }

}
