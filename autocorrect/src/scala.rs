// autocorrect: false
use super::*;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/scala.pest"]
struct ScalaParser;

#[allow(dead_code)]
pub fn format_scala(text: &str) -> code::FormatResult {
    let pairs = ScalaParser::parse(Rule::item, text);
    let text = code::FormatResult::new(text);
    code::format_pairs(text, pairs)
}

#[allow(dead_code)]
pub fn lint_scala(text: &str) -> code::LintResult {
    let pairs = ScalaParser::parse(Rule::item, text);
    let text = code::LintResult::new(text);
    code::format_pairs(text, pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_format_scala() {
        let example = r###"
/*
  多行注释第1行
  多行comment第2行
*/
object Test {
  val str1: String = "字符串Hello你好"
  var string_literal: String = s"$str1你好"

  var multiline_str: String = """这是str第1行
  这是str第2行"""
  
  // 正则regexp不应该处理
  val pattern = new Regex("[\w]1正则")
  val pattern1 = "1正则".r

  def main(args: Array[String]) {
    println( greeting )
  }
}
"###;

        let expect = r###"
/*
  多行注释第 1 行
  多行 comment 第 2 行
*/
object Test {
  val str1: String = "字符串 Hello 你好"
  var string_literal: String = s"$str1你好"

  var multiline_str: String = """这是 str 第 1 行
  这是 str 第 2 行"""
  
  // 正则 regexp 不应该处理
  val pattern = new Regex("[\w]1正则")
  val pattern1 = "1正则".r

  def main(args: Array[String]) {
    println( greeting )
  }
}
"###;

        assert_eq!(expect, format_scala(example).to_string());
    }
}
