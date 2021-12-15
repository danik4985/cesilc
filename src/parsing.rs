use std::collections::HashMap;

pub fn parse_data(lines: Vec<String>) -> Vec<i64> {
	let mut data: Vec<i64> = Vec::new();
	let mut in_data_mode = false;

	for i in lines {
		if i == "%" {
			in_data_mode = true;
		} else if i == "*" {
			in_data_mode = false;
		} else if in_data_mode {
			data.push(i.parse::<i64>().unwrap());
		}
	}

	return data;
}

#[derive(Clone)]
pub struct Instruction {
	pub inst: String,
	pub arg: String,
	pub err: Option<String>
}

pub struct ParseResult {
	pub map: HashMap<String, Vec<Instruction>>,
	pub order: Vec<String>
}

fn get_instruction(data: String) -> Instruction {
	// println!("{}", data);

	if data.starts_with("PRINT") {
		// Remove first 5 characters from data
		let mut data = data.clone().split_off(5);

		data = data.trim().to_string();

		if data.starts_with("\"") && data.ends_with("\"") {
			// println!("{}", data);

			let mut chars = data.chars();

			chars.next();
			chars.next_back();

			data = chars.collect::<String>();

			return Instruction {
				inst: "PRINT".to_string(),
				arg: data,
				err: None
			};

		} else {
			println!("Error in print data: {} || Not a valid string!", data);

			return Instruction {
				inst: "PRINT".to_string(),
				arg: data,
				err: Some("Invalid PRINT argument".to_string())
			};
		}
	}	else {
		let mut data = data.clone();

		data = data.trim().to_string();

		let split = data.split_whitespace().map(|i| i.to_string()).collect::<Vec<String>>();

		if split.len() == 1 {
			return Instruction {
				inst: split[0].clone(),
				arg: "".to_string(),
				err: None
			};
		} else {
			return Instruction {
				inst: split[0].clone(),
				arg: split[1].clone(),
				err: None
			};
		}
	}
}

pub fn parse_instructions(lines: Vec<String>) -> ParseResult {
	let mut map: HashMap<String, Vec<Instruction>> = HashMap::new();
	let mut order: Vec<String> = Vec::new();

	let mut in_data_mode = false;
	let mut current_label = "main".to_string();

	order.push(current_label.clone());
	map.insert("main".to_string(), Vec::new());

	for i in lines {
		if i == "%" {
			in_data_mode = true;
		} else if i == "*" {
			in_data_mode = false;
		} else if !in_data_mode {
			
			let mut data = i.clone();

			if !(data.starts_with(" ") || data.starts_with("\t")) {
				// We have a label
				let label = data.split_whitespace().next().unwrap().to_string();

				// println!("{}", label);

				current_label = label.clone();
				order.push(label.clone());
				data = data.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
			}

			let inst = get_instruction(data.trim().to_string());
			
			// Get the vector for the current label, add the instruction and put it back in the map
			let vec = map.entry(current_label.clone()).or_insert(Vec::new());
			vec.push(inst);
			let v2 = vec.iter().map(|i| i.to_owned()).collect::<Vec<Instruction>>();
			map.insert(current_label.clone(), v2);
			// My brain hurts
		}
	}

	return ParseResult { map, order };
}

pub fn to_lines(data: String) -> Vec<String> {
	return crate::preprocessor::preprocess(
		data
			.split("\n")
			.map(|i| i.to_string())
			.filter(|i| !i.trim().starts_with("("))
			.filter(|i| i.trim() != "")
			.collect()
	);
}
