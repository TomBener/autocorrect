// autocorrect: false
use crate::result::LineResult;
use owo_colors::AnsiColors::{Black, Green, Red, Yellow};
use owo_colors::OwoColorize;
use std::fmt::Write;

/// Diff a LineResult with colorize output.
pub(crate) fn diff_line_result(line: &LineResult, no_diff_bg_color: bool) -> String {
    let mut err_color = Red;
    if line.severity.is_warning() {
        err_color = Yellow;
    }

    let on_color = match no_diff_bg_color {
        true => on_color_transparent,
        false => on_color,
    };

    diff_lines_with_err_color(line.old.trim(), line.new.trim(), err_color, on_color)
}

#[allow(dead_code)]
pub(crate) fn diff_lines(old_str: &str, new_str: &str) -> String {
    diff_lines_with_err_color(old_str, new_str, Red, on_color)
}

type OnColorFn = fn(s: char, color: owo_colors::AnsiColors) -> String;

#[inline]
fn on_color(s: char, color: owo_colors::AnsiColors) -> String {
    let t = s.on_color(color).color(Black);
    format!("{t}")
}

#[inline]
fn on_color_transparent(s: char, color: owo_colors::AnsiColors) -> String {
    let t = s.color(color);
    format!("{t}")
}

/// Diff two strings and colorize the output.
///
/// Screenshot:
/// https://raw.githubusercontent.com/johannhof/difference.rs/master/assets/github-style.png
///
/// Forked from:
/// https://github.com/johannhof/difference.rs/blob/master/examples/github-style.rs
#[allow(unused_must_use)]
pub(crate) fn diff_lines_with_err_color(
    old_str: &str,
    new_str: &str,
    err_color: owo_colors::AnsiColors,
    on_color: OnColorFn,
) -> String {
    let diffs = diff::lines(old_str, new_str);

    let mut out = String::new();

    for i in 0..diffs.len() {
        match diffs[i] {
            diff::Result::Both(x, _) => {
                out.push(' ');
                out.push_str(x);
                out.push('\n');
            }
            // -
            diff::Result::Left(x) => {
                match diffs.get(i + 1) {
                    Some(diff::Result::Right(y)) => {
                        write!(out, "{}", "-".color(err_color));
                        let sub_diffs = diff::chars(y, x);
                        for c in sub_diffs {
                            match c {
                                diff::Result::Both(z, _) => {
                                    write!(out, "{}", z.color(err_color));
                                }
                                diff::Result::Right(z) => {
                                    write!(out, "{}", on_color(z, err_color));
                                }
                                _ => (),
                            }
                        }
                        out.push('\n');
                    }
                    _ => {
                        writeln!(out, "{}", format!("-{x}").color(err_color));
                    }
                };
            }
            // +
            diff::Result::Right(x) => {
                match diffs.get(i - 1) {
                    Some(diff::Result::Left(y)) => {
                        write!(out, "{}", "+".color(Green));
                        let sub_diffs = diff::chars(y, x);

                        for c in sub_diffs {
                            match c {
                                diff::Result::Both(z, _) => {
                                    write!(out, "{}", z.color(Green));
                                }
                                diff::Result::Right(z) => {
                                    write!(out, "{}", on_color(z, Green));
                                }
                                _ => (),
                            }
                        }
                        out.push('\n');
                    }
                    _ => {
                        writeln!(out, "{}", format!("+{x}").color(Green));
                    }
                };
            }
        }
    }

    // leave a blank line between each diff
    out.push('\n');

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::AnsiColors::{Red, Yellow};

    #[test]
    fn test_color_output() {
        let old_str = " Hello你好 ";
        let new_str = "Hallo 你好";

        let diff = diff_lines_with_err_color(old_str, new_str, Red, on_color);
        assert_eq!(
            "\u{1b}[31m-\u{1b}[39m\u{1b}[30;41m \u{1b}[0m\u{1b}[31mH\u{1b}[39m\u{1b}[30;41me\u{1b}[0m\u{1b}[31ml\u{1b}[39m\u{1b}[31ml\u{1b}[39m\u{1b}[31mo\u{1b}[39m\u{1b}[31m你\u{1b}[39m\u{1b}[31m好\u{1b}[39m\u{1b}[30;41m \u{1b}[0m\n\u{1b}[32m+\u{1b}[39m\u{1b}[32mH\u{1b}[39m\u{1b}[30;42ma\u{1b}[0m\u{1b}[32ml\u{1b}[39m\u{1b}[32ml\u{1b}[39m\u{1b}[32mo\u{1b}[39m\u{1b}[30;42m \u{1b}[0m\u{1b}[32m你\u{1b}[39m\u{1b}[32m好\u{1b}[39m\n\n",
            diff
        );

        let diff = diff_lines_with_err_color(old_str, new_str, Yellow, on_color);
        assert_eq!(
            "\u{1b}[33m-\u{1b}[39m\u{1b}[30;43m \u{1b}[0m\u{1b}[33mH\u{1b}[39m\u{1b}[30;43me\u{1b}[0m\u{1b}[33ml\u{1b}[39m\u{1b}[33ml\u{1b}[39m\u{1b}[33mo\u{1b}[39m\u{1b}[33m你\u{1b}[39m\u{1b}[33m好\u{1b}[39m\u{1b}[30;43m \u{1b}[0m\n\u{1b}[32m+\u{1b}[39m\u{1b}[32mH\u{1b}[39m\u{1b}[30;42ma\u{1b}[0m\u{1b}[32ml\u{1b}[39m\u{1b}[32ml\u{1b}[39m\u{1b}[32mo\u{1b}[39m\u{1b}[30;42m \u{1b}[0m\u{1b}[32m你\u{1b}[39m\u{1b}[32m好\u{1b}[39m\n\n",
            diff
        );

        let diff = diff_lines_with_err_color(old_str, new_str, Red, on_color_transparent);
        assert_eq!("\u{1b}[31m-\u{1b}[39m\u{1b}[31m \u{1b}[39m\u{1b}[31mH\u{1b}[39m\u{1b}[31me\u{1b}[39m\u{1b}[31ml\u{1b}[39m\u{1b}[31ml\u{1b}[39m\u{1b}[31mo\u{1b}[39m\u{1b}[31m你\u{1b}[39m\u{1b}[31m好\u{1b}[39m\u{1b}[31m \u{1b}[39m\n\u{1b}[32m+\u{1b}[39m\u{1b}[32mH\u{1b}[39m\u{1b}[32ma\u{1b}[39m\u{1b}[32ml\u{1b}[39m\u{1b}[32ml\u{1b}[39m\u{1b}[32mo\u{1b}[39m\u{1b}[32m \u{1b}[39m\u{1b}[32m你\u{1b}[39m\u{1b}[32m好\u{1b}[39m\n\n", diff);

        let diff = diff_lines_with_err_color(old_str, new_str, Yellow, on_color_transparent);
        assert_eq!("\u{1b}[33m-\u{1b}[39m\u{1b}[33m \u{1b}[39m\u{1b}[33mH\u{1b}[39m\u{1b}[33me\u{1b}[39m\u{1b}[33ml\u{1b}[39m\u{1b}[33ml\u{1b}[39m\u{1b}[33mo\u{1b}[39m\u{1b}[33m你\u{1b}[39m\u{1b}[33m好\u{1b}[39m\u{1b}[33m \u{1b}[39m\n\u{1b}[32m+\u{1b}[39m\u{1b}[32mH\u{1b}[39m\u{1b}[32ma\u{1b}[39m\u{1b}[32ml\u{1b}[39m\u{1b}[32ml\u{1b}[39m\u{1b}[32mo\u{1b}[39m\u{1b}[32m \u{1b}[39m\u{1b}[32m你\u{1b}[39m\u{1b}[32m好\u{1b}[39m\n\n", diff);
    }
}
