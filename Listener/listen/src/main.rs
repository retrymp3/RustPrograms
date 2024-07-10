use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    for s in listener.incoming(){
        let s = s.unwrap();
        println!("Yo who dat?");
    }
}
