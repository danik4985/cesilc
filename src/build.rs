pub fn build_binary(path: String, source: String, optimize: bool) {
	let fpath = format!("..{}.~.c", path);
	let write_result = std::fs::write(fpath.clone(), source);

	if write_result.is_err() {
		println!("Error writing file");
		return;
	}

	let mut cmd = std::process::Command::new("gcc");

	cmd.arg(&fpath);
	cmd.arg("-o");
	cmd.arg(path);

	#[cfg(not(debug_assertions))]
	cmd.arg("-w"); // Disabled, but only when release :swag:

	if optimize {
		cmd.arg("-O3");
	}

	let cmd_result = cmd.spawn().unwrap().wait();

	if cmd_result.is_err() {
		println!("Error compiling file");
	}

	std::fs::remove_file(fpath).unwrap();
}