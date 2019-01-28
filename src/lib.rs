extern crate futures;
extern crate hyper;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};

pub fn main() {
    let (tx, rx) = futures::sync::oneshot::channel::<()>();
    let tx = Arc::new(Mutex::new(Some(tx)));
    let counter = Arc::new(AtomicUsize::new(0));
    let new_service = move || {
        let counter = counter.clone();
        let tx = tx.clone();
        service_fn_ok(move |req| match (req.method(), req.uri().path()) {
            (&Method::GET, "/count") => {
                let count = counter.load(Ordering::Acquire);
                Response::new(Body::from(format!("count:{}", count)))
            }
            (&Method::POST, "/count") => {
                let count = counter.fetch_add(1, Ordering::AcqRel);
                Response::new(Body::from(format!("count:{}", count)))
            }
            (&Method::POST, "/stop") => {
                if let Some(tx) = tx.lock().unwrap().take() {
                    tx.send(()).unwrap();
                }
                Response::new(Body::from("stop"))
            }
            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("not found"))
                .unwrap(),
        })
    };

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr)
        .serve(new_service)
        .with_graceful_shutdown(rx)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);
    rt::run(server);
}
