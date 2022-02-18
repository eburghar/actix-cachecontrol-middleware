use serde_vecmap::opt_vecmap;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
/// Control Cache behavior
pub struct CacheControl {
	// cache control instructions for paths matching a list prefix
	#[serde(default)]
	#[serde(with = "opt_vecmap")]
	pub prefixes: Option<Vec<(String, String)>>,
	// cache control instructions for paths matching a list suffix
	#[serde(default)]
	#[serde(with = "opt_vecmap")]
	pub suffixes: Option<Vec<(String, String)>>,
}

impl CacheControl {
	/// return the first cache-control value that match path as a prefix or as a suffix
	pub fn get_value(&self, path: &str) -> Option<&str> {
		if let Some(ref suffixes) = self.suffixes {
			for (suffix, value) in suffixes.iter() {
				if path.ends_with(suffix) {
					return Some(value);
				}
			}
		}
		if let Some(ref prefixes) = self.prefixes {
			for (prefix, value) in prefixes.iter() {
				if path.starts_with(prefix) {
					return Some(value);
				}
			}
		}
		None
	}
}

impl Default for CacheControl {
	fn default() -> Self {
		Self {
			prefixes: None,
			suffixes: None,
		}
	}
}
