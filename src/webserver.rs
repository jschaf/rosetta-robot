// Implements http://rosettacode.org/wiki/Hello_world/Web_server
// not_tested

use std::io::net::tcp::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener, IoResult};

fn handle_client(mut stream: TcpStream) -> IoResult<()> {
    let response =
b"HTTP/1.1 200 OK
Content-Type: text/html;
charset=UTF-8

<doctype !html>
<html>
    <head>
        <title>Bye-bye baby bye-bye</title>
        <style>
            body { background-color: #111 }
            h1 { font-size:4cm; text-align: center; color: black; text-shadow: 0 0 2mm red}
        </style>
    </head>
    <body>
        <h1>Goodbye, world!</h1>
    </body>
</html>";

    try!(stream.write(response));
    let _ = stream.close_write();
    Ok(())
}

fn main() {
    let (ip, port) = ("127.0.0.1", 80);
    let listener = TcpListener::bind(ip, port).unwrap();

    let mut acceptor = listener.listen();
    println!("Listening for connections on port {}", port);

    for stream in acceptor.incoming() {
        spawn(proc() {
            match handle_client(stream.unwrap()) {
                Ok(_) => println!("Response sent!"),
                Err(e) => println!("Failed sending response: {}!", e),
            }
        });
    }

    // close the socket server
    drop(acceptor);
}
