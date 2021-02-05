mod utils;

use wasm_bindgen::prelude::*;

use syntect::highlighting::{ThemeSet, Theme};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxSet, SyntaxReference};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, syntect-web!");
}

#[wasm_bindgen]
pub struct Highlighter {
	ps: SyntaxSet,
	theme: Theme,
	syntax: SyntaxReference,
}
#[wasm_bindgen]
impl Highlighter {
	pub fn new(theme_path: &str) -> Highlighter {
		let _ps = SyntaxSet::load_defaults_newlines();
		let _theme = ThemeSet::get_theme(theme_path).unwrap();
		let _syntax = _ps.find_syntax_by_extension("rs").unwrap().clone();
		Highlighter{ps:_ps, theme:_theme, syntax: _syntax}
	}
	pub fn set_syntax_from_extension(&mut self, syntax_extension: &str) {
		self.syntax = self.ps.find_syntax_by_extension(syntax_extension).unwrap().clone();
	}
	pub fn set_syntax_from_path(&mut self, syntax_path: &str) {
		self.syntax = self.ps.find_syntax_by_path(syntax_path).unwrap().clone();
	}
	pub fn highlight_to_html(&self, content: &str) -> String {
		highlighted_html_for_string(content, &self.ps, &self.syntax, &self.theme)
	}
	pub fn set_theme_from_path(&mut self, theme_path: &str) {
		self.theme = ThemeSet::get_theme(theme_path).unwrap();
	}
}
