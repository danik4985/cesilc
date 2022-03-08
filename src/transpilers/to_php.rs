use std::collections::HashMap;

use crate::naming_manager::NamingManager;
use crate::parsing::ParseResult;

fn escape_string(s: &str) -> String {
		s.chars()
				.map(|c| match c {
						'\n' => "\\n".to_string(),
						'\r' => "\\r".to_string(),
						'\t' => "\\t".to_string(),
						'\'' => "\\'".to_string(),
						'\"' => "\\\"".to_string(),
						'\\' => "\\\\".to_string(),
						_ => c.to_string(),
				})
				.collect()
}

fn jump_gen(fnc: String) -> String {
	return format!("goto {};", fnc);
}

fn to_php(result: ParseResult, data: Vec<i64>, interactive: bool) -> String {
	let mut out = String::new();
	let mut vars: HashMap<String, i32> = HashMap::new();
	let mut nmgr = NamingManager::new();

	for i in result.order {
		// println!("{}", i);

		let instructions = result.map[&i].clone();

		out += &format!("{}: // {}\n", nmgr.name(i.clone()), i.clone());

		for j in instructions {
			match j.inst.as_str() {
				"PRINT" => out += &format!("echo \"{}\";\n", escape_string(&j.arg)),
				"LINE" => out += &"echo $LINE;\n",
				"OUT" => out += &format!("echo $acm;\n"),
				"IN" => out += "$acm = array_shift($data);\n",

				"LOAD" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("$acm = {};\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("$acm = load_from_mem(\"{}\", $memory);\n", escape_string(&j.arg));
					}
				},
				"STORE" => {
					out += &format!("$memory[\"{}\"] = $acm;\n", escape_string(&j.arg))
				},

				"ADD" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("$acm += {};\n", parse.unwrap());
					} else {
						out += &format!("$acm += load_from_mem(\"{}\", $memory);\n", escape_string(&j.arg));
					}
				},
				"SUBTRACT" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("$acm -= {};\n", parse.unwrap());
					} else {
						out += &format!("$acm -= load_from_mem(\"{}\", $memory);\n", escape_string(&j.arg));
					}
				},
				"MULTIPLY" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("$acm *= {};\n", parse.unwrap());
					} else {
						out += &format!("$acm *= load_from_mem(\"{}\", $memory);\n", escape_string(&j.arg));
					}
				},
				"DIVIDE" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						let num = parse.unwrap();

						if num == 0 {
							println!("This code might attempt to divide by 0. In that case, an error message will be printed.");
							out += "_error_divide_by_zero_();\n";
						} else {
							out += &format!("$acm /= {};\n", &num);
						}

					} else {
						out += &format!("$_tmp = load_from_mem(\"{}\", $memory);
						$acm = divide_unsafe($acm, $_tmp);\n", escape_string(&j.arg));
					}
				},

				"JUMP" => out += &format!("{}\n", jump_gen(nmgr.name(j.arg))),
				"JIZERO" => out += &format!("if ($acm === 0) {{{}}}\n", jump_gen(nmgr.name(j.arg))),
				"JINEG" => out += &format!("if ($acm < 0) {{{}}}\n", jump_gen(nmgr.name(j.arg))),
				"HALT" => out += &"exit(0);\n",

				"UIN" => {
					if interactive {
						out += &"$acm = intval(readline());\n";
					} else {
						println!("To use the UIN inst, you need interactive mode enabled.");
						println!("-- Ignoring --")
					}
				}

				_ => {
					// Unknown instruction
					println!("Unknown instruction: {}", j.inst);
					println!("-- Ignoring --");
				}
			}

			// out += &"\n";
		}
	}

	// out += &"}\n //Fin.";

	out = format!(r#"
	<?php

	$_cesil_fn_ = function() {{
		$acm = 0;
		$data = array({});
		$memory = array();

		$LINE = "{}";

		function load_from_mem(string $name, $memory) {{
			if (isset($memory[$name])) {{
				return $memory[$name];
			}}
			return 0;
		}}

		function divide_unsafe(int $op1, int $op2) {{
			if ($op2 === 0) {{
				_error_divide_by_zero_();
			}}

			return $op1 / $op2;
		}}

		function _error_divide_by_zero_() {{
			echo "*** DIVISION BY ZERO ***{}";
			die();
		}}

		{}

	}};

	$_cesil_fn_();


	?>
	"#,
	data.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
	if interactive { "\\n" } else { "<br>" },
	if interactive { "\\n" } else { "<br>" },
	&out
	);

	return out.trim().to_string();
}

pub fn transpile_to_php() -> Box<dyn Fn(ParseResult, Vec<i64>, bool, bool) -> String> {
	return Box::new(|result, data, interactive, _| {
		return to_php(result, data, interactive);
	});
}