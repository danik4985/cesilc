pub fn print_help() {
	println!("Usage: cesilc [OPTIONS] <file>");
	println!("");
	println!("Options:");
	println!("  -i, --interactive    Interactive mode");
	println!("  -o, --output <file>  Output file");
	println!("  -O, --optimize       Optimize code");
	println!("  -h, --help           Print this help message");
	println!("");
	println!("Output types:");
	println!("  SOURCE_C (*.c)       C source code");
	println!("  SOURCE_BASH (*.sh)   Bash source code");
	println!("  SOURCE_JS (*.js)     Javascript source code");
	println!("  BINARY               Executable binary");
	println!("");
	println!("Example:");
	println!("  cesilc -i -o output.c input.ces");
	println!("");
	println!("Version: {}", include_str!("./assets/version.txt"));
	println!("");
	println!("Licensed under GPLv3   danik4985 2022");
}