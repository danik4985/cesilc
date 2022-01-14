pub struct Args {
	pub file: String,
	pub output: String,
	pub outtype: String,
	pub interactive: bool,
	pub optimize: bool,
	pub help: bool
}

// This function is terrible.
// I hate it.
pub fn parse_args() -> Args {
	let mut file = String::new();
	let mut output = String::new();
	let mut outtype = String::new();
	let mut interactive = false;
	let mut next_is_output = false;
	let mut optimize = false;
	let mut help = false;

	// Get the command line arguments
	let args: Vec<String> = std::env::args().collect();

	for i in args {
		if next_is_output {
			output = i.clone();
			next_is_output = false;

			let extension = output.split(".").last().unwrap();

			outtype = match extension.to_ascii_lowercase().as_str() {
				"c" => "SOURCE_C".to_string(),
				"sh" => "SOURCE_BASH".to_string(),
				"js" => "SOURCE_JS".to_string(),

				// TODO: Add more output types

				  "exe"
				| "app"
				| _ /* Linux binaries don't have an extension */
				  => "BINARY".to_string()
			};
			
			continue;
		}

		if i == "-i" || i == "--interactive" {
			interactive = true;
		} else if i == "-n" || i == "--no-interactive" {
			interactive = false;
		} else if i == "-o" || i == "--output" {
			next_is_output = true;
		} else if i == "-O" || i == "--optimize" {
			optimize = true;
		} else if i == "-h" || i == "--help" {
			help = true;
		} else {
			file = i.clone();
		}
	}

	return Args {
		file, output, outtype, interactive, optimize, help
	};
}
