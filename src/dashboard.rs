use crate::{PAYLOADS, TREE, layout::Source};
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use serde_json::{Value, json};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    time::Duration,
};

const BATCH_SIZE: u64 = 12;

fn node_json(index: usize, path: String, probability: f64) -> Value {
    let node = TREE.nodes[index];
    let (kind, count, children) = match node.source {
        Source::Branch {
            first_child,
            child_count,
            total_weight,
        } => {
            let children = TREE.children
                [first_child as usize..(first_child + child_count) as usize]
                .iter()
                .map(|&child| {
                    let child = child as usize;
                    let name = TREE.name(child);
                    node_json(
                        child,
                        if path.is_empty() {
                            name.into()
                        } else {
                            format!("{path}/{name}")
                        },
                        probability * f64::from(TREE.nodes[child].weight) / total_weight as f64,
                    )
                })
                .collect::<Vec<_>>();
            ("branch", None, children)
        }
        Source::Corpus(corpus) => (
            "dataset",
            Some(PAYLOADS.corpuses[corpus as usize].record_count),
            Vec::new(),
        ),
        Source::Generator(_) => ("generator", None, Vec::new()),
    };
    json!({
        "id": index, "name": TREE.name(index), "path": path,
        "weight": node.weight, "probability": probability,
        "kind": kind, "count": count, "children": children,
    })
}

fn outputs(index: usize, seed: u64, offset: u64) -> Result<Value, String> {
    let source = TREE.nodes.get(index).ok_or("unknown node")?.source;
    let end = offset
        .checked_add(BATCH_SIZE)
        .ok_or("offset is too large")?;
    let mut items = Vec::new();
    let (next, done) = match source {
        Source::Branch { .. } => return Err("select a leaf to view outputs".into()),
        Source::Generator(_) => {
            for position in offset..end {
                let sample_seed = seed.wrapping_add(position);
                let text = PAYLOADS.render(source, &mut StdRng::seed_from_u64(sample_seed))?;
                items.push(json!({"text": text, "seed": sample_seed.to_string()}));
            }
            (end, false)
        }
        Source::Corpus(corpus) => {
            let corpus = PAYLOADS.corpuses[corpus as usize];
            let count = u64::from(corpus.record_count);
            if offset > count {
                return Err("offset exceeds dataset length".into());
            }
            let mut order: Vec<_> = (0..corpus.record_count).collect();
            order.shuffle(&mut StdRng::seed_from_u64(seed));
            let next = end.min(count);
            let selected = &order[offset as usize..next as usize];
            let indexes = selected
                .iter()
                .map(|&record| (corpus.first_record + record) as usize)
                .collect::<Vec<_>>();
            for (&record, text) in selected.iter().zip(PAYLOADS.record_texts(&indexes)?) {
                items.push(json!({"text": text, "record": record + 1}));
            }
            (next, next == count)
        }
    };
    Ok(json!({"items": items, "next": next, "done": done}))
}

struct Response {
    status: &'static str,
    content_type: &'static str,
    body: String,
}

impl Response {
    fn json(result: Result<Value, String>) -> Self {
        let (status, value) = match result {
            Ok(value) => ("200 OK", value),
            Err(error) => ("400 Bad Request", json!({"error": error})),
        };
        Self {
            status,
            content_type: "application/json; charset=utf-8",
            body: value.to_string(),
        }
    }
}

fn output_query(query: &str) -> Result<Value, String> {
    let mut node = None;
    let mut seed = None;
    let mut offset = None;
    for field in query.split('&') {
        let (key, value) = field.split_once('=').ok_or("invalid query")?;
        let slot = match key {
            "node" => &mut node,
            "seed" => &mut seed,
            "offset" => &mut offset,
            _ => return Err("unknown query parameter".into()),
        };
        if slot.replace(value).is_some() {
            return Err("duplicate query parameter".into());
        }
    }
    outputs(
        node.ok_or("missing node")?
            .parse()
            .map_err(|_| "invalid node")?,
        seed.ok_or("missing seed")?
            .parse()
            .map_err(|_| "invalid seed")?,
        offset
            .ok_or("missing offset")?
            .parse()
            .map_err(|_| "invalid offset")?,
    )
}

fn route(target: &str) -> Response {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    let (content_type, body) = match path {
        "/" => (
            "text/html; charset=utf-8",
            include_str!("../dashboard/index.html"),
        ),
        "/dashboard.css" => (
            "text/css; charset=utf-8",
            include_str!("../dashboard/dashboard.css"),
        ),
        "/dashboard.js" => (
            "text/javascript; charset=utf-8",
            include_str!("../dashboard/dashboard.js"),
        ),
        "/api/tree" => return Response::json(Ok(node_json(0, String::new(), 1.0))),
        "/api/outputs" => return Response::json(output_query(query)),
        _ => {
            return Response {
                status: "404 Not Found",
                content_type: "text/plain; charset=utf-8",
                body: "Not found".into(),
            };
        }
    };
    Response {
        status: "200 OK",
        content_type,
        body: body.into(),
    }
}

fn connection(mut stream: TcpStream, port: u16) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
    let mut request = Vec::new();
    let mut buffer = [0; 1024];
    while !request.windows(4).any(|part| part == b"\r\n\r\n") {
        let count = stream.read(&mut buffer)?;
        if count == 0 || request.len() + count > 8192 {
            return Ok(());
        }
        request.extend_from_slice(&buffer[..count]);
    }
    let request = String::from_utf8_lossy(&request);
    let mut lines = request.lines();
    let mut first = lines.next().unwrap_or_default().split_whitespace();
    let method = first.next().unwrap_or_default();
    let target = first.next().unwrap_or_default();
    let host = lines.find_map(|line| {
        let (key, value) = line.split_once(':')?;
        key.eq_ignore_ascii_case("host").then_some(value.trim())
    });
    let response = if host != Some(format!("127.0.0.1:{port}").as_str()) {
        Response {
            status: "403 Forbidden",
            content_type: "text/plain",
            body: "Use the printed loopback URL".into(),
        }
    } else if method != "GET" {
        Response {
            status: "405 Method Not Allowed",
            content_type: "text/plain",
            body: "Use GET".into(),
        }
    } else {
        route(target)
    };
    write!(
        stream,
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: default-src 'self'; style-src 'self' 'unsafe-inline'; frame-ancestors 'none'\r\n\r\n{}",
        response.status,
        response.content_type,
        response.body.len(),
        response.body
    )
}

pub fn serve(port: u16) -> Result<(), String> {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|e| format!("cannot start dashboard on port {port}: {e}"))?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    println!("Dashboard: http://127.0.0.1:{port}/ (Ctrl+C to stop)");
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    for stream in listener.incoming() {
        // Keep rendering and decompression bounded to one request at a time.
        if let Err(error) = connection(stream.map_err(|e| e.to_string())?, port) {
            eprintln!("inspire dashboard: {error}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn tree_probabilities_follow_sibling_weights() {
        fn check(node: &Value) {
            let children = node["children"].as_array().unwrap();
            if children.is_empty() {
                return;
            }
            let total: f64 = children.iter().map(|c| c["weight"].as_f64().unwrap()).sum();
            for child in children {
                let expected = node["probability"].as_f64().unwrap()
                    * child["weight"].as_f64().unwrap()
                    / total;
                assert!((child["probability"].as_f64().unwrap() - expected).abs() < 1e-12);
                assert_eq!(
                    TREE.resolve(child["path"].as_str().unwrap()),
                    Some(child["id"].as_u64().unwrap() as usize)
                );
                check(child);
            }
        }
        check(&node_json(0, String::new(), 1.0));
    }

    #[test]
    fn datasets_page_through_a_complete_reproducible_permutation() {
        for (index, node) in TREE.nodes.iter().enumerate() {
            let Source::Corpus(corpus) = node.source else {
                continue;
            };
            let mut seen = HashSet::new();
            let mut offset = 0;
            loop {
                let batch = outputs(index, 42, offset).unwrap();
                assert_eq!(batch, outputs(index, 42, offset).unwrap());
                for item in batch["items"].as_array().unwrap() {
                    assert!(seen.insert(item["record"].as_u64().unwrap()));
                    assert!(!item["text"].as_str().unwrap().is_empty());
                }
                offset = batch["next"].as_u64().unwrap();
                if batch["done"].as_bool().unwrap() {
                    break;
                }
            }
            assert_eq!(
                seen.len(),
                PAYLOADS.corpuses[corpus as usize].record_count as usize
            );
            assert!(
                outputs(index, 42, offset).unwrap()["items"]
                    .as_array()
                    .unwrap()
                    .is_empty()
            );
        }
    }

    #[test]
    fn generators_continue_with_seeds_that_reproduce_cli_samples() {
        for (index, node) in TREE.nodes.iter().enumerate() {
            if !matches!(node.source, Source::Generator(_)) {
                continue;
            }
            for offset in [0, BATCH_SIZE] {
                let batch = outputs(index, u64::MAX, offset).unwrap();
                assert_eq!(batch["done"], false);
                assert_eq!(batch["next"], offset + BATCH_SIZE);
                for item in batch["items"].as_array().unwrap() {
                    let seed = item["seed"].as_str().unwrap().parse().unwrap();
                    assert_eq!(
                        item["text"],
                        PAYLOADS
                            .render(node.source, &mut StdRng::seed_from_u64(seed))
                            .unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn bad_queries_are_errors_and_assets_are_embedded() {
        for query in [
            "",
            "node=0&seed=1&offset=0",
            "node=999999&seed=1&offset=0",
            "node=1&seed=x&offset=0",
            "node=1&seed=1&offset=18446744073709551615",
            "node=1&node=2&seed=1&offset=0",
        ] {
            assert_eq!(
                route(&format!("/api/outputs?{query}")).status,
                "400 Bad Request"
            );
        }
        for path in ["/", "/dashboard.js", "/dashboard.css", "/api/tree"] {
            let response = route(path);
            assert_eq!(response.status, "200 OK");
            assert!(!response.body.is_empty());
        }
        assert_eq!(route("/missing").status, "404 Not Found");
    }
}
