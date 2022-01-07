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
	return format!("return '{}'", fnc);
}

fn to_js(result: ParseResult, data: Vec<i64>, _interactive: bool) -> String {
	let mut out = String::new();
	let mut vars: HashMap<String, i32> = HashMap::new();
	let mut nmgr = NamingManager::new();
	let mut first_iter = true;

	for i in result.order {
		// println!("{}", i);

		let instructions = result.map[&i].clone();

		if first_iter {
			out += "main() {\n";
			first_iter = false;
		} else {
			out += &format!("{} }},\n", jump_gen(nmgr.name(i.clone())));
			out += &format!("{}() {{\n", nmgr.name(i.clone()));
		}

		// out += &format!("{}: // {}\n", nmgr.name(i.clone()), i.clone());

		for j in instructions {
			match j.inst.as_str() {
				"PRINT" => out += &format!("print(\"{}\")\n", escape_string(&j.arg)),
				"LINE" => out += &"print(lineEnd)\n",
				"OUT" => out += &format!("print(String(acm))\n"),
				"IN" => out += "load_()\nif (acm === undefined) return\n",

				"LOAD" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm = {}\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm = data[{}]\n", vars[&j.arg]);
					}
				},
				"STORE" => {
					if !vars.contains_key(&j.arg) {
						vars.insert(j.arg.clone(), vars.len() as i32);
					}

					out += &format!("data[{}] = acm\n", vars[&j.arg])
				},

				"ADD" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm += {}\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm += data[{}]\n", vars[&j.arg]);
					}
				},
				"SUBTRACT" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm -= {}\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm -= data[{}]\n", vars[&j.arg]);
					}
				},
				"MULTIPLY" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm *= {}\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm *= data[{}]\n", vars[&j.arg]);
					}
				},
				"DIVIDE" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						let num = parse.unwrap();

						if num == 0 {
							println!("This code might attempt to divide by 0. In that case, an error message will be printed.");
							out += "_error_divide_by_zero_()\n";
						} else {
							out += &format!("acm /= {}\n", &num);
						}

					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("_tmp = data[{}]
						if (_tmp == 0) _error_divide_by_zero_()
						else acm /= _tmp\n", vars[&j.arg]);
					}

					out += &"if (!currentLabel) return\n";
				},

				"JUMP" => out += &format!("{}\n", jump_gen(nmgr.name(j.arg))),
				"JIZERO" => out += &format!("if (acm == 0) {{{}}}\n", jump_gen(nmgr.name(j.arg))),
				"JINEG" => out += &format!("if (acm < 0) {{{}}}\n", jump_gen(nmgr.name(j.arg))),
				"HALT" => out += &"return false\n",

				"UIN" => {
					println!("To use the UIN inst, you need interactive mode enabled. You can not have IMODE in JS!");
					println!("-- Ignoring --");
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

	out += &"}\n //Fin.";

	out = format!(
		"
		((module) => {{
			var print = null
			var lineEnd = '\\n'
			var currentLabel = 'main'
			var tmp
		
			const LABELS = {{
				{}
			}}
		
			const storage = [{}]
		
			const data = []
			var acm = 0
		
			function load_() {{
				if (storage.length == 0) {{
					print('*** PROGRAM REQUIRES MORE DATA ***\\n')
					currentLabel = false
				}}

				acm = storage.shift()
			}}
		
			function _error_divide_by_zero_() {{
				print('*** DIVISION BY ZERO ***\\n')
				currentLabel = false
			}}
		
			function core(printFnc) {{
				print = printFnc
				while (currentLabel) currentLabel = LABELS[currentLabel]()
			}}
		
			if (typeof window === 'undefined') {{
				if (/*require.main === module*/!module.parent) {{
					core(process.stdout.write.bind(process.stdout))
				}} else {{
					module.exports = (pfnc) => {{
						if (typeof pfnc === 'function') {{
							core(pfnc)
						}} else {{
							core(process.stdout.write.bind(process.stdout))
						}}
					}}
				}}
			}} else {{
				window.CESIL_RUN = (pfnc) => {{
					lineEnd = '<br>'
		
					if (typeof pfnc === 'string') {{
						const el = document.querySelector(pfnc)
						core(txt => el.innerHTML += txt)
					}} else {{
						core(txt => document.body.innerHTML += txt)
					}}
				}}
			}}
			
		}})(typeof window !== 'undefined' ? window : module.exports)",
		&out,
		data.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ")
	);

	return out;
}

pub fn transpile_to_js() -> Box<dyn Fn(ParseResult, Vec<i64>, bool, bool) -> String> {
	return Box::new(|result, data, interactive, _| {
		return to_js(result, data, interactive);
	});
}