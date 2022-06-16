req 0.1.0

Rest API tester just in your terminal. happy testing! :)

USAGE:

    req [OPTIONS] --url <URL>

OPTIONS:

        --IB                      include the body in the output (default: false) , (if the body is
                                  too large, it will be truncated)
                                  
        --IH                      include the headers in the output (default: false)
        
        --IS <STATUS>             include the status code and status message  in the output
                                  (default: true) [default: 0]
                                  
                                  
    -B, --body <BODY>...          body to send to the server (for POST and PUT) (optional)
                                  (multiple) , example : req --method post --url
                                  http://localhost:8080/  --body '{"name":"John"}'
                                  
    -h, --help                    Print help information
    
    -H, --headers <HEADERS>...    headers to send to the server (optional)(multiple) , example : req
                                  --method post --url http://localhost:8080/  --headers
                                  '{"Content-Type":"application/json"}'
                                  
    -M, --method <METHOD>         Method to use for the request (get, post, put, delete) (default:
                                  get) [default: get]
                                  
    -Q, --query <QUERY>...        query string to send to the server (optional) , example : req
                                  --method get --url http://localhost:8080/  --query '?name=John'
                                  
    -U, --url <URL>               url to use for the request (required) (e.g. http://www.google.com)
    
    -V, --version                 Print version information
