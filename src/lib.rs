#![cfg_attr(windows, feature(abi_vectorcall))]

use std::path::PathBuf;
use ext_php_rs::prelude::*;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::{css_for_theme_with_class_style, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::html::ClassStyle;

/// Replies with Bar
///
/// @return string Bar
#[php_function]
pub fn rust_foo() -> String {
    String::from("Bar")
}


#[php_function]
pub fn syntect_highlight(code: &str, language_ext: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_token(language_ext).unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut html_generator = ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
    for line in LinesWithEndings::from(code) {
        html_generator.parse_html_for_line_which_includes_newline(line).unwrap();
    }
    let output_html= html_generator.finalize();
    return output_html;
}

#[php_function]
pub fn syntect_css(theme_path: &str) -> String {

    let theme = ThemeSet::get_theme(PathBuf::from(theme_path)).unwrap();
    css_for_theme_with_class_style(&theme, ClassStyle::Spaced).unwrap()
}

// Required to register the extension with PHP.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
