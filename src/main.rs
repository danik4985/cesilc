/*
Cesil compiler/transpiler
Copyright (C) 2021 danik4985

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use transpilers::{to_sh::transpile_to_sh, to_js::transpile_to_js, to_c::transpile_to_c};

mod args;
mod parsing;
mod utils;
mod transpilers;
mod naming_manager;
mod build;
mod preprocessor;

fn main() {
	let args = args::parse_args();
	let source_raw = std::fs::read_to_string(args.file).unwrap();
	let lines = parsing::to_lines(source_raw.clone());
	let data = parsing::parse_data(lines.clone());
	let instructions = parsing::parse_instructions(lines.clone());

	// println!("{:?}", lines);

	// Done? Let's clean up!
	utils::drop(source_raw);
	utils::drop(lines);

	// Transpilation time!
	let transpile_function = match args.outtype.as_str() {
		"SOURCE_C" => transpile_to_c(),
		"SOURCE_BASH" => transpile_to_sh(),
		"BINARY" => transpile_to_c(),
		"SOURCE_JS" => transpile_to_js(),

		_ => transpile_to_c()
	};

	let transpiled = transpile_function(instructions, data, args.interactive, args.optimize);

	// println!("{}", transpiled);

	if args.outtype == "BINARY" {
		build::build_binary(args.output, transpiled, args.optimize);
	} else {
		std::fs::write(args.output, transpiled).unwrap();
	}

	println!("Done!");
}
