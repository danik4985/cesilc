use std::collections::HashMap;

use crate::{parsing::{ParseResult, Instruction}, naming_manager::NamingManager};

fn escape_string(s: &str) -> String {
	s.chars()
			.map(|c| match c {
					'\n' => "\\n".to_string(),
					'\r' => "\\r".to_string(),
					'\t' => "\\t".to_string(),
					'\'' => "\\'".to_string(),
					'\"' => "\\\"".to_string(),
					'\\' => "\\\\".to_string(),
					'$'  => "\\$".to_string(),
					_ => c.to_string(),
			})
			.collect()
}


fn to_sh(result: ParseResult, data: Vec<i64>, interactive: bool, optimize: bool) -> String {
	let mut out = String::new();
	let mut labels_index = String::new();
	let mut ic = 0;
	let mut labels = NamingManager::new();
	let mut vars: HashMap<String, i32> = HashMap::new();
	let mut skip: i8 = 0;
	let mut inst: Vec<Instruction> = Vec::new();

	for i in &result.order {
		inst.append(&mut result.map[i].clone());
	}

	for i in result.order {
		labels_index += &format!("{} {}\n", labels.name(i.clone()), ic);

		for j in &result.map[&i] {
			if skip > 0 {
				skip -= 1;
				continue;
			}

			out += &format!("i{}(){{ ", ic);

			match j.inst.as_str() {
				"PRINT" => out += &format!("echo -n \"{}\";", escape_string(&j.arg)),
				"LINE" => out += &format!("echo;"),
				"OUT" => out += &format!("echo -n \"$ACM\";"),
				"IN" => out += &format!("_input_;"),

				"LOAD" => {
					let parse = j.arg.parse::<i32>();

					// Optimize
					if ( optimize &&
						inst[ic + 1].inst == "ADD"
						|| inst[ic + 1].inst == "SUBTRACT"
						|| inst[ic + 1].inst == "MULTIPLY"
						|| inst[ic + 1].inst == "DIVIDE")
						&& inst[ic + 2].inst == "STORE"
						&& inst[ic + 3].inst == "LOAD" {
						skip = 2;

						// println!("Skipping {}", ic);
						
						let calc_op = inst[ic + 1].clone();
						let store_op = inst[ic + 2].clone();
						let parse2 = calc_op.arg.parse::<i32>();
						let num1 = if parse.is_ok() {
							parse.unwrap().to_string()
						} else {
							if !vars.contains_key(&j.arg) {
								vars.insert(j.arg.clone(), vars.len() as i32);
							}

							format!("${{data[{}]}}", vars[&j.arg])
						};
						let num2 = parse2.unwrap();
						let store_loc = {
							if !vars.contains_key(&store_op.arg) {
								vars.insert(store_op.arg.clone(), vars.len() as i32);
							}
									
							vars[&store_op.arg]
						};

						if calc_op.inst == "DIVIDE" && num2 == 0 {
							println!("This code might attempt to divide by 0. In that case, an error message will be printed.");
							out += "_error_divide_by_zero_;\n";
						} else {
							out += &format!(
								"data[{}]=$(({} {} {})); }}\n",
								store_loc,
								&num1,
								match calc_op.inst.as_str() {
									"ADD" => "+",
									"SUBTRACT" => "-",
									"MULTIPLY" => "*",
									_ => panic!("Unknown operation"),
								},
								&num2
							);
						}

						ic += 1;
						continue;
					}

					if parse.is_ok() {
						out += &format!("ACM=$(({}));", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("ACM=$(echo ${{data[{}]}});", vars[&j.arg]);
					}
				},
				"STORE" => {
					if !vars.contains_key(&j.arg) {
						vars.insert(j.arg.clone(), vars.len() as i32);
					}

					out += &format!("data[{}]=$(echo $ACM);", vars[&j.arg])
				},

				"ADD" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("ACM=$((ACM + {}));", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("ACM=$((ACM + ${{data[{}]}}));", vars[&j.arg]);
					}
				},
				"SUBTRACT" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("ACM=$((ACM - {}));", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("ACM=$((ACM - ${{data[{}]}}));", vars[&j.arg]);
					}
				},
				"MULTIPLY" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("ACM=$((ACM * {}));", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("ACM=$((ACM * ${{data[{}]}}));", vars[&j.arg]);
					}
				},
				"DIVIDE" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						let num = parse.unwrap();

						if num == 0 {
							println!("This code might attempt to divide by 0. In that case, an error message will be printed.");
							out += "_error_divide_by_zero_;\n";
						} else {
							out += &format!("ACM=$((ACM / {}));\n", &num);
						}

					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("_division_ ${{data[{}]}};\n", vars[&j.arg]);
					}
				},

				"JUMP" => out += &format!("_jmp {};", labels.name(j.arg.clone())),
				"JINEG" => out += &format!("[[ $ACM -lt 0 ]] && _jmp {};", labels.name(j.arg.clone())),
				"JIZERO" => out += &format!("[[ $ACM -eq 0 ]] && _jmp {};", labels.name(j.arg.clone())),
				"HALT" => out += "exit;",

				"UIN" => {
					if interactive {
						out += &format!("read ACM;");
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

			out += "}\n";

			ic += 1;
		}
	}

	out = format!(
		"
		#!/bin/bash

		data=()
		pdata=( {} )
		_dp_=0
		DPM={}
		MI={}

		function _input_ {{
			if [[ $_dp_ -lt $DPM ]]; then ACM=$(echo ${{pdata[$_dp_]}}); ((_dp_++)); else 
			echo \"*** PROGRAM REQUIRES MORE DATA ***\"
			exit
			fi
		}}

		function _division_ {{
			if [[ $1 -eq 0 ]]; then
				echo \"*** DIVISION BY ZERO ***\"
				exit 1
			fi

			ACM=$((ACM / $1))
		}}

		function _jmp {{
			_id=$(echo \"{}\" | grep \"$1\" | awk '{{ print $2 }}')
			instp=$((_id - 1))
		}}

		{}
		
		instp=0

		# Generated by CESILc v1.0.0
		# Learn more about CESIL at https://en.wikipedia.org/wiki/CESIL

		while ((instp < MI)); do
			# echo \">> $instp <<\"
			\"i$instp\"
			((instp++))
		done
		",
		data.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "),
		data.len(),
		&ic,
		labels_index,
		&out
	).trim_start().to_string();

	return out;
}

pub fn transpile_to_sh() -> Box<dyn Fn(ParseResult, Vec<i64>, bool, bool) -> String> {
	return Box::new(|result, data, interactive, optimize| {
		return to_sh(result, data, interactive, optimize);
	});
}