use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};


struct CacheData {
    data: String
}

struct Counter {
    count: i32
}

impl CacheData {
    fn save_data(&mut self, data: String) {
        self.data = data;
    }
}

impl  Counter {
    fn add (&mut self) {
        self.count = self.count + 1;
    }
    fn reset_count(&mut self) {
        self.count = 0;
    }
}

fn main() -> std::io::Result<()> {
    let mut cache: CacheData = CacheData {data: String::from("")};
    let mut number_count: Counter = Counter {count: 0};
    let addr: TcpListener = TcpListener::bind("127.0.0.1:3001").unwrap();
    
    for stream in addr.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut number_count, &mut cache);
    }
    Ok(())
}


fn handle_connection(mut stream: TcpStream, counter: &mut Counter, cache: &mut CacheData) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let get_route: String = buf_reader.lines().next().unwrap().unwrap();
    if counter.count == 0 {
        let data: String = fs::read_to_string("../../scrapping/data.json").unwrap();
        cache.save_data(data);
    }
    let data_json_to_response = &cache.data;
    
    if  get_route == "GET /stocks HTTP/1.1" {
        let status_ok: &str = "HTTP/1.1 200 OK";
        let response: String = format!(
            "{status_ok}\r\n\r\n{data_json_to_response}"
        );
        counter.add();
        stream.write_all(response.as_bytes()).unwrap();

        if counter.count == 10 {
            counter.reset_count();
        }
    }

   
} 
