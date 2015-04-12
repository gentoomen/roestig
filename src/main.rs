extern crate serialize;

use serialize::json;
use std::error::Error;
use std::fs:File;

struct Connection {
	server: &str,
	port: i32,
	channels: mut Vec<&str>
}

struct BotContext {
	nick: mut &str,
	username: &str
}

impl BotContext {
	fn read_config_file(config_filename: &str) {
		let mut file = try!(File::open(config_filename));
		let mut raw_json = String::new();

		try!(file.read_to_string(&mut raw_json));

		// TODO: Figure out how the heck to parse a JSON string
	}
}

fn main() {
}
