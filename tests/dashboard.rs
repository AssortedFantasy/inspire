use serde_json::Value;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    process::{Child, Command, Stdio},
    time::Duration,
};

struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn request(port: u16, target: &str, host: &str, method: &str) -> (String, String) {
    let mut socket = TcpStream::connect(("127.0.0.1", port)).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    write!(socket, "{method} {target} HTTP/1.1\r\nHost: {host}\r\n\r\n").unwrap();
    let mut response = String::new();
    socket.read_to_string(&mut response).unwrap();
    let (headers, body) = response.split_once("\r\n\r\n").unwrap();
    (headers.into(), body.into())
}

#[test]
fn dashboard_serves_embedded_assets_and_real_outputs_without_source_files() {
    let mut server = Server(
        Command::new(env!("CARGO_BIN_EXE_inspire"))
            .current_dir(std::env::temp_dir())
            .args(["--dashboard", "--port", "0"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let mut line = String::new();
    BufReader::new(server.0.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    let port: u16 = line
        .split("127.0.0.1:")
        .nth(1)
        .unwrap()
        .split('/')
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let host = format!("127.0.0.1:{port}");
    for asset in ["/", "/dashboard.js", "/dashboard.css"] {
        let (headers, body) = request(port, asset, &host, "GET");
        assert!(headers.starts_with("HTTP/1.1 200 OK"));
        assert!(!body.is_empty());
        assert!(headers.contains(&format!("Content-Length: {}\r\n", body.len())));
    }
    let (_, body) = request(port, "/api/tree", &host, "GET");
    let tree: Value = serde_json::from_str(&body).unwrap();
    fn find_generator(node: &Value) -> Option<&Value> {
        if node["kind"] == "generator" {
            return Some(node);
        }
        node["children"]
            .as_array()
            .unwrap()
            .iter()
            .find_map(find_generator)
    }
    let generator = find_generator(&tree).unwrap();
    let (_, body) = request(
        port,
        &format!("/api/outputs?node={}&seed=42&offset=0", generator["id"]),
        &host,
        "GET",
    );
    let batch: Value = serde_json::from_str(&body).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_inspire"))
        .args([
            "--family",
            generator["path"].as_str().unwrap(),
            "--seed",
            "42",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        format!("{}\n", batch["items"][0]["text"].as_str().unwrap())
    );
    assert_eq!(batch["items"].as_array().unwrap().len(), 12);
    assert!(
        request(port, "/api/outputs?node=nope", &host, "GET")
            .0
            .starts_with("HTTP/1.1 400")
    );
    assert!(
        request(port, "/", "external.invalid", "GET")
            .0
            .starts_with("HTTP/1.1 403")
    );
    assert!(
        request(port, "/", &host, "POST")
            .0
            .starts_with("HTTP/1.1 405")
    );
    assert!(
        request(port, "/missing", &host, "GET")
            .0
            .starts_with("HTTP/1.1 404")
    );
}
