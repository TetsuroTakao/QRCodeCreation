use qrcode::QrCode;
use qrcode::render::svg;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use wasm_bindgen::prelude::*;
use std::str;

fn main()
{
    let url = "http://blog.processtune.com/";
    let xml = create_qrcode_svg(url);
    if cfg!(debug_assertions){
        println!("{}", xml);
    } else {
        let listener:TcpListener = TcpListener::bind("127.0.0.1:7879").unwrap();
        for stream in listener.incoming() {
            let stream: TcpStream = stream.unwrap();
            handle_connection(stream, xml.as_str());
        }
    }
}

#[wasm_bindgen]
pub fn create_qrcode_svg(_url: &str) -> String{
    let code:QrCode = QrCode::new(_url).unwrap();
    code.render::<svg::Color>().build()
}

fn handle_connection(mut stream: TcpStream, content: &str) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request_content = str::from_utf8(&buffer).unwrap();
    let mut request_header:Vec<&str> = Vec::new();
    for line in request_content.lines(){
        if line.contains("HTTP") {
            let v: Vec<&str> = line.split(' ').collect();
            request_header.push(v[0]);
            request_header.push(v[1]);
        } else if line.contains(":") {
            let kv: Vec<&str> = line.split(":").collect();
            if kv[0] == "Host"{
                request_header.push(kv[1]);
                request_header.push(kv[2]);
            }
        } else if line.len() == 0 {
        } else {
            let _body = line;
        }
    }
    let contents = format!("<!DOCTYPE html><html lang=\"ja\"><head><meta charset=\"utf-8\"><title>{}</title></head><body><div>【メソッド】{} 【ホスト】{} 【ポート】{} 【パス】{}</div><p>{}</p><p>{}</p></body></html>","デバッグ画面",request_header[0],request_header[2],request_header[3],request_header[1],content,request_content);
    let status_line = "HTTP/1.1 201 QR Code created";
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}