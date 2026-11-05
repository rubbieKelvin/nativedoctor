use std::time::{Duration, Instant};

use nd_core::{PreparedBody, PreparedRequest};
use nd_engine::{send, Error, Event};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

fn request(port: u16, timeout: Option<Duration>) -> PreparedRequest {
    return PreparedRequest {
        method: "GET".to_string(),
        url: format!("http://127.0.0.1:{port}/").parse().unwrap(),
        headers: Vec::new(),
        body: PreparedBody::None,
        timeout,
        follow_redirects: false,
        max_redirects: 30,
        verify_tls: true,
    };
}

/// Sends a response head, then each chunk after a delay, then closes.
async fn server(content_type: &'static str, chunks: Vec<(u64, &'static str)>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buffer = [0; 1024];
        let _ = socket.read(&mut buffer).await.unwrap();
        let head = format!(
            "HTTP/1.1 200 OK\r\ncontent-type: {content_type}\r\ntransfer-encoding: chunked\r\n\r\n"
        );
        socket.write_all(head.as_bytes()).await.unwrap();
        for (delay, text) in chunks {
            tokio::time::sleep(Duration::from_millis(delay)).await;
            let frame = format!("{:x}\r\n{text}\r\n", text.len());
            if socket.write_all(frame.as_bytes()).await.is_err() {
                return;
            }
        }
        let _ = socket.write_all(b"0\r\n\r\n").await;
    });
    return port;
}

#[tokio::test]
async fn chunks_arrive_as_they_are_sent() {
    let port = server("text/plain", vec![(0, "first"), (400, "second")]).await;
    let (sender, mut events) = mpsc::channel(8);
    let task = tokio::spawn(send(request(port, None), sender));

    let mut first_chunk_at = None;
    let mut last_chunk_at = None;
    let mut text = String::new();
    let mut finished = false;
    assert!(matches!(
        events.recv().await,
        Some(Event::Head { status: 200, .. })
    ));
    while let Some(event) = events.recv().await {
        match event {
            Event::Chunk(bytes) => {
                first_chunk_at.get_or_insert_with(Instant::now);
                last_chunk_at = Some(Instant::now());
                text.push_str(std::str::from_utf8(&bytes).unwrap());
            }
            Event::Finished { .. } => finished = true,
            Event::Head { .. } => panic!("only one head expected"),
        }
    }
    task.await.unwrap().unwrap();

    assert!(finished);
    assert_eq!(text, "firstsecond");
    let gap = last_chunk_at.unwrap() - first_chunk_at.unwrap();
    assert!(
        gap >= Duration::from_millis(300),
        "chunks were buffered: {gap:?}"
    );
}

#[tokio::test]
async fn a_stalled_body_times_out() {
    let port = server("text/plain", vec![(0, "first"), (5_000, "late")]).await;
    let (sender, mut events) = mpsc::channel(8);
    let task = tokio::spawn(send(
        request(port, Some(Duration::from_millis(100))),
        sender,
    ));
    while events.recv().await.is_some() {}
    assert!(matches!(task.await.unwrap(), Err(Error::Timeout(_))));
}

#[tokio::test]
async fn a_zero_timeout_means_no_limit() {
    let port = server("text/plain", vec![(200, "ok")]).await;
    let (sender, mut events) = mpsc::channel(8);
    let task = tokio::spawn(send(request(port, Some(Duration::ZERO)), sender));
    while events.recv().await.is_some() {}
    task.await.unwrap().unwrap();
}

#[tokio::test]
async fn dropping_the_receiver_cancels_quietly() {
    let port = server("text/plain", vec![(0, "first"), (5_000, "late")]).await;
    let (sender, mut events) = mpsc::channel(1);
    let task = tokio::spawn(send(request(port, None), sender));
    assert!(events.recv().await.is_some());
    drop(events);

    let result = tokio::time::timeout(Duration::from_secs(2), task).await;
    let finished = result.expect("send should stop after the receiver is dropped");
    finished.unwrap().unwrap();
}

#[tokio::test]
async fn refused_connections_are_connect_errors() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let (sender, _events) = mpsc::channel(8);
    let result = send(request(port, None), sender).await;
    assert!(matches!(result, Err(Error::Connect(_))), "{result:?}");
}
