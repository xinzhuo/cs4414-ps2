//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::{str, int, io, path};

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count:int = 0;

fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        println(fmt!("Request received:\n%s", request_str));
			let mut response = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
			println("Test");
			let mut endIndex = 4;

				for request_str.iter().skip(5).advance() |c| {
					endIndex += 1;
					if c == ' ' {break;}
				};
							
			let path = request_str.slice(5,endIndex).trim();
            println(fmt!("path:%s | path_length:%u", path, path.len()));
			unsafe {
				if path.is_empty() {
				visitor_count += 1;
		              	 response = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
			         <doctype !html><html><head><title>Hello, Rust!</title>
			         <style>body { background-color: #111; color: #FFEEAA }
			         h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm 	red}
		           	</style></head>
			        <body>
			        <h1>Greetings, Rusty!</h1>
				<p>Visitor Count:" + int::to_str(visitor_count) + "</p> 
			        </body></html>\r\n"
				}
				else { let file_content = loadFile(path.to_str());
						
						for file_content.iter().advance |content| {
							println(fmt!("%?",content.trim()));
							response = response + content.trim();
						}
						response.trim();
						response = response + "\r\n";
					 }
				
			}
			println(fmt!("The response is: %s", response));			
			
		       	net_tcp::write(&sock, response.as_bytes_with_null_consume());
                    },
                };
            }
        }
    };
}

fn loadFile(filename: ~str)-> ~[~str] {
	let file_reader: Result<@Reader, ~str>;
	file_reader = io::file_reader(~path::Path(filename));
	match file_reader {
		Ok(file) => { file.read_lines() }
		Err(error) => { fail!("Cannot open file: " + error); }
	}
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}


