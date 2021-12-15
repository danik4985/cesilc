/*
CESIL
From Wikipedia, the free encyclopedia
Jump to navigation
Jump to search
Not to be confused with Cecil (programming language).

CESIL, or Computer Education in Schools Instruction Language,[1] is a programming language designed to introduce pupils in British secondary schools to elementary computer programming. It is a simple language containing a total of fourteen instructions.
Contents

    1 Background
    2 Structure
    3 Instructions
        3.1 Input and Output
        3.2 Memory storage
        3.3 Mathematical instructions
        3.4 Program control
        3.5 Other symbols
    4 CESIL Programming Tools
    5 Example
    6 See also
    7 Bibliography
    8 Notes
    9 References

Background

Computer Education in Schools (CES) was a project developed in the late 1960s and early 1970s by International Computers Limited (ICL).[2] CESIL was developed by ICL as part of the CES project, and introduced in 1974.[3] In those days, very few if any schools had computers, so pupils would write programs on coding sheets, which would then be transferred to punched cards or paper tape.[4] Typically, this would be sent to run on a mainframe computer, with the output from a line printer being returned later.[5]
Structure

Because CESIL was not designed as an interactive language, there is no facility to input data in real time. Instead, numeric data is included as a separate section at the end of the program.[6]

The fundamental principle of CESIL is the use of a single accumulator, which handles mathematical operations.[4] Numeric values are stored in variables, which in CESIL are referred to as store locations.[7] CESIL only works with integers, and results from DIVIDE operations are rounded if necessary.[8] There is no facility for structured data such as arrays, nor for string handling, though string constants can be output by means of the PRINT instruction.[4]

Jumps and loops can be conditional or non-conditional, and transfer operation of the program to a line with a specific label, which is identified in the first column of a coding sheet.[9] The instruction or operation is stated in the second column, and the operand in the third column.[10] On some coding sheets, comments and the text of the PRINT instruction would be written in a fourth column.[11]
Instructions

Instructions, or operations, are written in upper case and may have a single operand, which can be a store location, constant integer value or line label. Store locations and line labels are alphanumeric, up to six characters, and begin with a letter.[12] Numeric integer constants must be signed + or −, with zero being denoted as +0.[13][a]
Input and Output

    IN – reads the next value from the data, and stores it in the accumulator.[4] The error message *** PROGRAM REQUIRES MORE DATA *** is printed if the program tries to read beyond the end of the data provided.[14]
    OUT – prints the current value of the accumulator. No carriage return is printed.[15]
    PRINT "text in quotes" – prints the given text. No carriage return is printed.[15]
    LINE – prints a carriage return, thus starting a new line.[16]

Memory storage

    LOAD location or LOAD constant – copies the value of the given location or constant to the accumulator.[17]
    STORE location – copies the contents of the accumulator to the given location.[10]

Mathematical instructions

    ADD location or ADD constant – adds the value of the given location or constant to the accumulator.[18]
    SUBTRACT location or SUBTRACT constant – subtracts the value of the given location or constant from the accumulator.[19]
    MULTIPLY location or MULTIPLY constant – multiplies the accumulator by the value of the given location or constant.[20]
    DIVIDE location or DIVIDE constant – divides the accumulator by the value of the given location or constant.[21] The result is rounded down if the result is positive, and up if the result is negative.[8] A *** DIVISION BY ZERO *** error message is printed if the divisor is zero.[22]

In each case, the result of the operation is stored in the accumulator, replacing the previous value.
Program control

    JUMP label – unconditionally transfers control to location labelled.[23]
    JINEG label (Jump If NEGative) – transfers control to location labelled if the accumulator contains a negative value.[24]
    JIZERO label (Jump If ZERO) – transfers control to location labelled if the accumulator contains zero.[9]
    HALT – terminates the program.[15]

Other symbols

Three special symbols are used in CESIL at the beginnings of lines.

    % is used to mark the end of the program and the start of data.[11]
    * is used to mark the end of the data.[25]
    ( is used at the start of a line to indicate a comment.[26][b]

CESIL Programming Tools

An emulator for CESIL, designed to run on Windows and called Visual CESIL, is available as freeware.[27]

An interpreter for CESIL, designed to run on the Android platform and called Wyrm CESIL, is available as free to install.[28]
Example

The following totals the integers in the runtime data section until it encounters a negative value and prints the total.

        LOAD    +0
LOOP    STORE   TOTAL
        IN
        JINEG   DONE
        ADD     TOTAL
        JUMP    LOOP

DONE    PRINT   "The total is: "
        LOAD    TOTAL
        OUT
        LINE
        HALT

%
1
2
3
-1
*

The output of the above program would be:

The total is: 6 
*/

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

fn to_c(result: ParseResult, data: Vec<i64>, interactive: bool) -> String {
	let mut out = String::new();
	let mut vars: HashMap<String, i32> = HashMap::new();
	let mut nmgr = NamingManager::new();

	for i in result.order {
		// println!("{}", i);

		let instructions = result.map[&i].clone();

		out += &format!("{}: // {}\n", nmgr.name(i.clone()), i.clone());

		for j in instructions {
			match j.inst.as_str() {
				"PRINT" => out += &format!("printf(\"{}\");\n", escape_string(&j.arg)),
				"LINE" => out += &"printf(\"\\n\");\n",
				"OUT" => out += &format!("printf(\"%d\", acm);\n"),
				"IN" => out += "load_();\n",

				"LOAD" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm = {};\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm = data[{}];\n", vars[&j.arg]);
					}
				},
				"STORE" => {
					if !vars.contains_key(&j.arg) {
						vars.insert(j.arg.clone(), vars.len() as i32);
					}

					out += &format!("data[{}] = acm;\n", vars[&j.arg])
				},

				"ADD" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm += {};\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm += data[{}];\n", vars[&j.arg]);
					}
				},
				"SUBTRACT" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm -= {};\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm -= data[{}];\n", vars[&j.arg]);
					}
				},
				"MULTIPLY" => {
					let parse = j.arg.parse::<i32>();

					if parse.is_ok() {
						out += &format!("acm *= {};\n", parse.unwrap());
					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("acm *= data[{}];\n", vars[&j.arg]);
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
							out += &format!("acm /= {};\n", &num);
						}

					} else {
						if !vars.contains_key(&j.arg) {
							vars.insert(j.arg.clone(), vars.len() as i32);
						}

						out += &format!("_tmp = data[{}];
						if (_tmp == 0) _error_divide_by_zero_();
						else acm /= _tmp;\n", vars[&j.arg]);
					}
				},

				"JUMP" => out += &format!("{}\n", jump_gen(nmgr.name(j.arg))),
				"JIZERO" => out += &format!("if (acm == 0) {{{}}}\n", jump_gen(nmgr.name(j.arg))),
				"JINEG" => out += &format!("if (acm < 0) {{{}}}\n", jump_gen(nmgr.name(j.arg))),
				"HALT" => out += &"exit(0);\n",

				"UIN" => {
					if interactive {
						out += &"_read_input_();\n";
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

	out = format!(
		"
	#include <stdio.h>
	#include <stdlib.h>

	int acm = 0;
	int data[{}];
	int _pd_[] = {{{}}};
	int _pdp_ = 0;
	int _tmp = 0;

	#define _PDL_ {}

	void _error_divide_by_zero_() {{
		printf(\"*** DIVISION BY ZERO ***\\n\");
		exit(1);
	}}

	void _error_out_of_data_() {{
		printf(\"*** PROGRAM REQUIRES MORE DATA ***\\n\");
		exit(1);
	}}

	{}

	void load_() {{
		if (_pdp_ < _PDL_) {{
			acm = _pd_[_pdp_];
			_pdp_++;
		}} else {{
			_error_out_of_data_();
		}}
	}}

	int main() {{
		/*
					Generated by CESILc v1.0.0
					Learn more about CESIL at https://en.wikipedia.org/wiki/CESIL
		*/

		{}

		return 0;
	}}",
		vars.len(),
		data.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
		data.len(),
		if interactive {
			" // only present when interactive mode is enabled
				void _read_input_() {
					scanf(\"%d\", &acm);
				}
		"
		} else { "" },
		out
	);

	return out;
}

pub fn transpile_to_c() -> Box<dyn Fn(ParseResult, Vec<i64>, bool) -> String> {
	return Box::new(|result, data, interactive| {
		return to_c(result, data, interactive);
	});
}