use reqwest;
use reqwest::blocking::Client;

// globals
static  USER_AGENT :&str  = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";

// struct

#[derive(Debug)]
pub struct MyClient{
    client  : Client
}

impl MyClient {
    pub fn new() -> Self{
        let  client : Client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .build().expect("can not build the client structure");
        MyClient { client }
    }

    pub fn get_req(self,url:&str) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>>{
        let  res = self.client.get(url).send()?;
        Ok(res)
    }


    pub fn post_req(self,url : &str) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>>{
        let res = self.client.post(url)
        .body("helo world")
        .send()?;
        Ok(res)
    }
}
