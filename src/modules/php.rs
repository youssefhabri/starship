use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Create a segment with the current PHP version
///
/// Will display the PHP version if any of the following criteria are met:
///     - Current directory contains a `composer.json` file
///     - Current directory contains a `.php` file
pub fn segment(context: &Context) -> Option<Module> {
    let is_php_project = context
        .new_scan_dir()
        .set_files(&["composer.json"])
        .set_extensions(&["php"])
        .scan();

    if !is_php_project {
        return None;
    }

    match get_php_version() {
        Some(php_version) => {
            const PHP_CHAR: &str = "🐘 ";
            let module_color = Color::Blue.bold();

            let mut module = Module::new("php");
            module.set_style(module_color);

            let formatted_version = format_php_version(php_version)?;
            module.new_segment("symbot", PHP_CHAR);
            module.new_segment("version", formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_php_version() -> Option<String> {
    Command::new("php")
        .arg("-v")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
}

fn format_php_version(php_stdout: String) -> Option<String> {
    // "PHP (cli) ..."
    let version = php_stdout
        // split into ["", ["7.2.17 (cli)"]
        .splitn(2, "PHP")
        // return "7.2.17 (cli)"
        .nth(1)?
        // split into ["7.2.17", "(cli)", ...]
        .split_whitespace()
        // return "7.2.17"
        .next()?
        // split into ["7.2.17", "0ubuntu0.18.04.1"]
        // needed for version string with a dash (e.g PHP 7.2.17-0ubuntu0.18.04.1 (cli) ...)
        .splitn(2, "-")
        // return "7.2.17"
        .next()?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_php_version() {
        let input_with_dash =
            String::from("PHP 7.2.17-0ubuntu0.18.04.1 (cli) (built: Apr 18 2019 14:12:38) ( NTS )");
        assert_eq!(
            format_php_version(input_with_dash),
            Some("v7.2.17".to_string())
        );

        let input_without_dash =
            String::from("PHP 7.2.17 (cli) (built: Apr 18 2019 14:12:38) ( NTS )");
        assert_eq!(
            format_php_version(input_without_dash),
            Some("v7.2.17".to_string())
        );
    }
}
