$:.push File.expand_path("lib", __dir__)

Gem::Specification.new do |s|
  s.name = "autocorrect-rb"
  s.version = "2.1.1.beta1"
  s.platform = Gem::Platform::RUBY
  s.authors = ["Jason Lee"]
  s.email = ["huacnlee@gmail.com"]
  s.homepage = "https://github.com/huacnlee/autocorrect"
  s.summary = "AutoCorrect is a linter and formatter to help you to improve copywriting."
  s.description = "AutoCorrect is a linter and formatter to help you to improve copywriting, correct spaces, words, punctuations between CJK (Chinese, Japanese, Korean)."
  s.files = Dir.glob("lib/**/*") + %w[README.md Rakefile]
  s.require_paths = ["lib"]
  s.license = "MIT"
  s.required_ruby_version = ">= 2.7.0"

  # needed until rubygems supports Rust support is out of beta
  s.add_dependency "rb_sys", ">= 0.9.18"
end