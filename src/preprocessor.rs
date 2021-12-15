pub fn preprocess(lines: Vec<String>) -> Vec<String> {
	let mut result = Vec::new();

	for line in lines {
		// println!("{}", line);

		if line.starts_with("#") {
			// println!("sussers");

			let mut line = line.clone();

			line.remove(0).to_string();

			// println!("{}", line);

			if line.starts_with("include") {
				let line = line.trim_start_matches("include").trim();
				result.append(&mut crate::parsing::to_lines(std::fs::read_to_string(line).unwrap()));
			}
		} else {
			result.push(line);
		}
	}

	return result;
}