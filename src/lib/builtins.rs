const TOO_MANY_ARGS: &'static str = "Error: Too many arguments were supplied";

pub fn cd(args: &[&str]) {
	use std::{env, path};

	let path = match args.len() {
		 0 => match env::home_dir() {
			Some(path) => path,
			None => return,
		},
		1 => path::PathBuf::from(args[0]),
		_ => {
			eprintln!("{}", TOO_MANY_ARGS);
			return;
		},
	};

	match env::set_current_dir(path) {
		Ok(_) => {},
		Err(err) => eprintln!("{}", err),
	}
}

pub fn exit(args: &[&str]) {
	use std::process;

	match args.len() {
		0 => process::exit(0),
		1 => match args[0].parse::<u8>() {
			Ok(sta) => process::exit(sta as i32),
			Err(err) => eprintln!("{}", err),
		},
		_ => eprintln!("{}", TOO_MANY_ARGS),
	}
}
