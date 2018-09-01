use std::env;
extern crate github_rs;
extern crate serde_json;

use github_rs::client::{Executor, Github};
use serde_json::Value;

fn main() {
	let args: Vec<String> = env::args().collect();
	let api_token = &args[1];
	let a = &api_token[0..3];
	let b = &api_token[api_token.len()-3..api_token.len()];
	println!("Executing with API token {}...{}", a, b);

	let client = Github::new(api_token).unwrap();
	let me = client.get().user().execute::<Value>();

	match me {
		Ok((headers, status, json)) => {
			println!("{}", headers);
			println!("{}", status);
			if let Some(json) = json {
				println!("{}", json);
			}
		},
		Err(e) => println!("{}" , e)
	}
}
