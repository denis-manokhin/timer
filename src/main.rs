use std::{
	env,
	io::{self, Write},
	thread, time,
};

struct WaitTime {
	hours: u32,
	minutes: u32,
	seconds: u32,
}

fn main() {
	start_timer(&parse_args())
}

fn start_timer(wait_time: &WaitTime) {
	let total_duration_sec = wait_time.seconds + wait_time.minutes * 60 + wait_time.hours * 60 * 60;
	let second = time::Duration::from_secs(1);

	print_progress(0);
	for n in 1..total_duration_sec + 1 {
		thread::sleep(second);
		print_progress(n);
	}

	println!()
}

fn print_progress(seconds_pass: u32) {
	let hours = seconds_pass / 3600;
	let minutes = (seconds_pass % 3600) / 60;
	let seconds = seconds_pass % 60;

	print!(
		"\rPass {} hours {} minutes {} seconds",
		hours, minutes, seconds
	);
	io::stdout().flush().unwrap();
}

fn parse_args() -> WaitTime {
	let mut parsed_hours: u32 = 0;
	let mut parsed_minutes: u32 = 0;
	let mut parsed_seconds: u32 = 0;

	let arguments: Vec<String> = env::args().collect();
	for (index, argument) in arguments.iter().enumerate() {
		if argument == "-h" {
			parsed_hours = parse_arg_or_exit(String::from("hours"), arguments[index + 1].clone());
		}
		if argument == "-m" {
			parsed_minutes = parse_arg_or_exit(String::from("minutes"), arguments[index + 1].clone());
		}
		if argument == "-s" {
			parsed_seconds = parse_arg_or_exit(String::from("seconds"), arguments[index + 1].clone());
		}
	}

	return WaitTime {
		hours: parsed_hours,
		minutes: parsed_minutes,
		seconds: parsed_seconds,
	};
}

fn parse_arg_or_exit(arg_name: String, arg_value: String) -> u32 {
	let parsed = arg_value.parse::<u32>();
	if parsed.is_err() {
		println!("invalid {} number", arg_name);
		std::process::exit(1)
	}

	return parsed.unwrap();
}
