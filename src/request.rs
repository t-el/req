use std::collections::HashMap;
use crate::parser::Args;
use reqwest::{Method, header,RequestBuilder};
use reqwest::header::HeaderMap;
// what is the best anime ever 


/// parse the header vector to a header map
pub fn get_headers<'a>(headers:&'a Vec<String>) -> Option<&'a HeaderMap> {
    let mut header_map = HeaderMap::new();
    let headers = headers.iter().map(|header| {
        if !header.contains(":") {
            return ;
        }
        let header_split = header.split(":").collect::<Vec<&str>>();
        let key = header_split[0];
        let value = header_split[1];
        header_map.insert(header::HeaderName::from_static(key), header::HeaderValue::from_static(value));
    });
    Some(&header_map)
}



#[derive(Debug)]
pub struct CmdRequestBuilder {
  args: Args,
  header_map : Option<HeaderMap>,
}

impl CmdRequestBuilder {
    pub fn new(args:Args) -> Self {
        CmdRequestBuilder {
         args:args,
         header_map : None,
        }
    }

    // convert self.method to reqwest::Method
    pub fn get_method(&self) -> Method {
        match self.args.method.as_str() {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            "delete" => Method::DELETE,
            _ => Method::GET,
        }
    }


   

    // build the request with the given method, url, query, body, headers
    pub fn build(&self) -> RequestBuilder {
     let builder = reqwest::Client::new().request(self.get_method(), self.args.url.as_str());
      // add headers
        let header_map = get_headers(self.args.headers.as_ref().unwrap());
    


        todo!()
    }
   


    
}


       