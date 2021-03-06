use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn tcp_v1() {
    // bind 函數返回 Result<T, E>，這表明綁定可能會失敗
    // 因為我們是出於學習目的來編寫一個基礎的服務器，將不用關心處理這類錯誤，使用 unwrap 在出現這些情況時直接停止程式。
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 有時會看到對於一次瀏覽器請求會列印出多條資訊；這可能是因為瀏覽器在請求頁面的同時還請求了其他資源
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 在 handle_connection 中，stream 參數是可變的。這是因為 TcpStream 實例在內部記錄了所返回的數據。
    // 它可能讀取了多於我們請求的數據並儲存它們以備下一次請求數據。因此它需要是 mut 的因為其內部狀態可能會改變

    // 創建了一個 1024 字節的緩沖區，它足以存放基本請求的數據
    let mut buffer = [0; 1024];

    // 接著將緩沖區傳遞給 stream.read ，它會從 TcpStream 中讀取字節並放入緩沖區中。
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        // let response = "HTTP/1.1 200 OK\r\n\r\n";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
fn handle_connection_v2(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // 接著將緩沖區傳遞給 stream.read ，它會從 TcpStream 中讀取字節並放入緩沖區中。
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 獲取到連接時列印資訊，現在調用新的 handle_connection 函數並向其傳遞 stream。
        handle_connection_v2(stream);
    }
}
