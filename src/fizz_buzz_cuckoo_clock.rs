fn fizz_buzz_cuckoo_clock(time:&str) -> String {
	let hour: u8 = time[0..2].parse().unwrap();
	let minute: u8 = time[3..5].parse().unwrap();
	format_result(hour, minute)
}

fn format_result(hour:u8, minute:u8) -> String {
	if minute == 0 {
		return format_hour(hour);
	} else if minute == 30 {
		"Cuckoo"
	} else if minute % 15 == 0 {
		"Fizz Buzz"
	} else if minute % 3 == 0 {
		"Fizz"
	} else if minute % 5 == 0 {
		"Buzz"
	} else {
		"tick"
	}.to_string()
}

fn format_hour(hour:u8) -> String {
	let count = if hour == 0 {
		12
	} else if hour > 12 {
		hour - 12
	} else {
		hour
	};
	"Cuckoo ".repeat(count as usize - 1) + "Cuckoo"
}

#[cfg(test)]
mod tests {
	use super::fizz_buzz_cuckoo_clock;

	#[test]
	fn test0(){
		let test_cases  = vec![
			("13:34", "tick"),
			("21:00", "Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo"),
			("11:15", "Fizz Buzz"),
			("03:03", "Fizz"),
			("14:30", "Cuckoo"),
			("08:55", "Buzz"),
			("00:00", "Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo"),
			("12:00", "Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo Cuckoo"),
		];
		for (input, want) in test_cases {
			let got = fizz_buzz_cuckoo_clock(input);
			assert_eq!(want, got);
		}
	}
}