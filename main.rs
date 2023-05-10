use std::net::TcpStream;
use std::thread;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

fn main() {
    let id = std::io::stdin().read_line(&mut String::new()).unwrap().trim().to_owned();
    let vanity = std::io::stdin().read_line(&mut String::new()).unwrap().trim().to_owned();
    let token = std::io::stdin().read_line(&mut String::new()).unwrap().trim().to_owned();

    let mut threads = Vec::new(); // use 10 threads for this user can increase on compile
    for _ in 0..10 {
        let id = id.clone();
        let vanity = vanity.clone();
        let token = token.clone();
        let t = thread::spawn(move || check_vanity(id, vanity, token));
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn check_vanity(id: String, vanity: String, token: String) { //check function
    loop {
        match TcpStream::connect("discord.com:443") {
            Ok(mut stream) => {
                stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap(); //after 10 sec knock it off

                let mut headers = HeaderMap::new();
                headers.insert(
                    HeaderName::from_static("Authorization"),
                    HeaderValue::from_str(&token).unwrap(),
                );
                headers.insert(
                    HeaderName::from_static("Content-Type"),
                    HeaderValue::from_static("application/json"),
                );

                let response = reqwest::blocking::Client::new()
                    .get(&format!("https://discord.com/api/v9/invites/{}", vanity))
                    .headers(headers)
                    .send();

                match response {
                    Ok(res) => {
                        match res.status().as_u16() {
                            404 => {
                                println!("Vanity {} is available", vanity);
                                let res = reqwest::blocking::Client::new()
                                    .patch(&format!("https://discord.com/api/v9/guilds/{}/vanity-url", id))
                                    .headers(headers)
                                    .json(&json!({"code": vanity}))
                                    .send();

                                if res.map_or(false, |r| r.status().is_success()) {
                                    println!("Vanity successfully claimed");
                                    return;
                                } else {
                                    println!("Failed");
                                }
                            },
                            200 => println!("Vanity {} is taken.", vanity),
                            _ => println!("Error checking vanity {}", vanity),
                        }
                    },
                    Err(_) => println!("Error connecting to Discord:443 and Discord:80"),
                }
            },
            Err(_) => continue,//handle errors with out unwrapping
        }
    }
}
