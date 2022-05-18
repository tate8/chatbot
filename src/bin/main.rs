extern crate chat_app;
use chat_app::ThreadPool;

use std::io::prelude::*;
use std::fs::File;
use std::net::
{
    TcpListener,
    TcpStream,
};

const NUM_THREADS: usize = 4;


fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").expect("Couldn't connect to the server...");
   let pool = ThreadPool::new(NUM_THREADS);

   for stream in listener.incoming()
   {
       let stream = stream.unwrap();

       pool.execute(|| { handle_connection(stream); });
       
   }
}

// read data from the TCP stream and handle it
fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    // some http request headers
    let get_index = b"GET / HTTP/1.1\r\n";
    let get_message = b"GET /message HTTP/1.1\r\n";
    let get_styles = b"GET /styles.css HTTP/1.1\r\n";
    let get_js = b"GET /index.js HTTP/1.1\r\n";


    // request to get index.html
    if buffer.starts_with(get_index)
    {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let filename = "templates/index.html";

        write_file_to_buffer(filename, status_line, &mut stream)
    }
    // request to get styles.css
    else if buffer.starts_with(get_styles)
    {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let filename = "static/css/styles.css";

        write_file_to_buffer(filename, status_line, &mut stream)
    }
    // request to get index.js
    else if buffer.starts_with(get_js)
    {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let filename = "static/js/index.js";

        write_file_to_buffer(filename, status_line, &mut stream)
    }
    // request to get message.html
    else if buffer.starts_with(get_message)
    {
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let filename = "templates/message.html";

        write_file_to_buffer(filename, status_line, &mut stream)
    }
    else
    {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let filename = "templates/404.html";

        write_file_to_buffer(filename, status_line, &mut stream)
    }
}


// write file to buffer to be sent as a HTTPResponse
fn write_file_to_buffer(filename: &str, status_line: &str, stream: &mut TcpStream)
{
    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}