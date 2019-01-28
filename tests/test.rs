extern crate rewrite;
extern crate reqwest;

static HOST: &str = "http://127.0.0.1:8080/";

fn post(path: &str) -> String {
    let url = HOST.to_owned() + path;
    reqwest::Client::new().post(&url).send().unwrap().text().unwrap()
}

fn get(path: &str) -> String {
    let url = HOST.to_owned() + path;
    reqwest::get(&url).unwrap().text().unwrap()
}

#[test]
fn test() {
    let server = std::thread::spawn(|| rewrite::main());
    assert_eq!(get("count"), "count:0");
    assert_eq!(post("count"), "count:0");
    assert_eq!(get("count"), "count:1");
    assert_eq!(post("count"), "count:1");
    assert_eq!(get("count"), "count:2");
    post("stop");
    server.join().unwrap();
}
