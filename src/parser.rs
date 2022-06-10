use std::env;
use req::MyClient;

#[derive(Debug)]
pub enum  CommandType {
    Get,
    Post,
    Put,
    Delete,
}

impl  CommandType {
   pub fn from_str(command_type: &str) -> Result<Self, &'static str> {
         match command_type {
              "get" => Ok(CommandType::Get),
              "post" => Ok(CommandType::Post),
              "put" => Ok(CommandType::Put),
              "delete" => Ok(CommandType::Delete),
              _ => Err("Unknown command type")
         }
    }
}
    


#[derive(Debug)]
pub struct Command {
    pub command : CommandType,
    pub target_url : String,
}


// implementation
impl Command {

    pub fn new(command : CommandType, target_url : String) -> Self {
        Command {
            command,
            target_url,
        }
    }

    

    pub fn get_target_url(&self) -> String {
        self.target_url.clone()
    }
    
 
    pub fn get_command_string(&self) -> String {
        match self.command {
            CommandType::Get => "get",
            CommandType::Post => "post",
            CommandType::Put => "put",
            CommandType::Delete => "delete",
        }
        .to_string()
    }

    pub fn get_command_string_with_url(&self) -> String {
        format!("{} {}", self.get_command_string(), self.get_target_url())
    }

    pub fn execute(&self, body: &str) {
      // if body is empty, then it is a get request
      // if body is not empty, then it is a post request with a body to be sent
      // if body is not empty, then it is a put request with a body to be sent
      // if body is not empty, then it is a delete request with a body to be sent

        let client = MyClient::new();
        let url = self.get_target_url();
      
        
        let res = match self.command {
            CommandType::Get => client.get_req(&url),
            CommandType::Post => client.post_req(&url, body),
            CommandType::Put => client.put_req(&url, body),
            CommandType::Delete => client.delete_req(&url, body),

           
        };
        match res {
            Ok(res) => {
                println!("{}", res.text().unwrap());
            }
            Err(e) => {
                println!("{}", e);
            }
        }


    }


}

