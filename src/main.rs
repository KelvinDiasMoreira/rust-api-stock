use std::{ fs, io::{BufRead, BufReader}, net::{TcpListener, TcpStream}};

#[path = "./controller/controllers.rs"] mod controllers;



fn main() -> std::io::Result<()> {
    let addr: TcpListener = TcpListener::bind("127.0.0.1:3001").unwrap();
    
    for stream in addr.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let get_route: String = buf_reader.lines().next().unwrap().unwrap();

    if  get_route == "GET /stocks HTTP/1.1" {
        controllers::handle_get_stocks(stream, &fs::read_to_string("../../scrapping/data.json").unwrap());
    }
}

