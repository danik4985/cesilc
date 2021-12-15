use std::collections::HashMap;

pub struct NamingManager {
	table: HashMap<String, usize>
}

impl NamingManager {
	pub fn new() -> NamingManager {
		NamingManager {
			table: HashMap::new()
		}
	}

	pub fn name(&mut self, name: String) -> String {
		if !self.table.contains_key(&name) {
			self.table.insert(name.clone(), self.table.len());
		}

		return format!("x_{}", self.table[&name].to_string());
	}

}
