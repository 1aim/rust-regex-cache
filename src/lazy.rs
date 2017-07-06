// Copyright 2017 1aim GmbH
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is furnished to do
// so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::ops::Deref;

use regex::{Regex, RegexBuilder, Error};
use syntax::Expr;
use thread_local::CachedThreadLocal;

/// A lazily created `Regex`.
///
/// At the first `Deref` the given source will be compiled and saved in the
/// Local Thread Storage, thus avoiding locking.
#[derive(Debug)]
pub struct Lazy {
	builder: Builder,
	local:   CachedThreadLocal<Regex>,
}

impl Lazy {
	/// Create a new lazy `Regex` for the given source, checking the syntax is
	/// valid.
	pub fn new(source: &str) -> Result<Lazy, Error> {
		if let Err(err) = Expr::parse(source) {
			return Err(err.into());
		}

		Ok(Lazy {
			builder: Builder::new(source),
			local:   Default::default(),
		})
	}
}

impl Deref for Lazy {
	type Target = Regex;

	fn deref(&self) -> &Regex {
		self.local.get_or(||
			Box::new(RegexBuilder::new(&self.builder.source)
				.case_insensitive(self.builder.case_insensitive)
				.multi_line(self.builder.multi_line)
				.dot_matches_new_line(self.builder.dot_matches_new_line)
				.swap_greed(self.builder.swap_greed)
				.ignore_whitespace(self.builder.ignore_whitespace)
				.unicode(self.builder.unicode)
				.size_limit(self.builder.size_limit)
				.dfa_size_limit(self.builder.dfa_size_limit)
				.build().unwrap()))
	}
}

impl Clone for Lazy {
	fn clone(&self) -> Lazy {
		Lazy {
			builder: self.builder.clone(),
			local:   Default::default(),
		}
	}
}

/// A configurable builder for a lazy `Regex`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Builder {
	source: String,
	case_insensitive: bool,
	multi_line: bool,
	dot_matches_new_line: bool,
	swap_greed: bool,
	ignore_whitespace: bool,
	unicode: bool,
	size_limit: usize,
	dfa_size_limit: usize,
}

impl Default for Builder {
	fn default() -> Self {
		Builder {
			source: "".into(),
			case_insensitive: false,
			multi_line: false,
			dot_matches_new_line: false,
			swap_greed: false,
			ignore_whitespace: false,
			unicode: true,
			size_limit: 10 * (1 << 20),
			dfa_size_limit: 2 * (1 << 20),
		}
	}
}

impl Builder {
	/// Create a new regular expression builder with the given pattern.
	///
	/// If the pattern is invalid, then an error will be returned when
	/// `compile` is called.
	pub fn new(source: &str) -> Builder {
		Builder {
			source: source.to_owned(),

			.. Default::default()
		}
	}

	/// Consume the builder and compile the regular expression.
	///
	/// Note that calling `as_str` on the resulting `Regex` will produce the
	/// pattern given to `new` verbatim. Notably, it will not incorporate any
	/// of the flags set on this builder.
	pub fn build(&self) -> Result<Lazy, Error> {
		if let Err(err) = Expr::parse(&self.source) {
			return Err(err.into());
		}

		Ok(Lazy {
			builder: self.clone(),
			local:   Default::default(),
		})
	}

	/// Set the value for the case insensitive (`i`) flag.
	pub fn case_insensitive(&mut self, yes: bool) -> &mut Builder {
		self.case_insensitive = yes;
		self
	}

	/// Set the value for the multi-line matching (`m`) flag.
	pub fn multi_line(&mut self, yes: bool) -> &mut Builder {
		self.multi_line = yes;
		self
	}

	/// Set the value for the any character (`s`) flag, where in `.` matches
	/// anything when `s` is set and matches anything except for new line when
	/// it is not set (the default).
	///
	/// N.B. "matches anything" means "any byte" for `regex::bytes::Regex`
	/// expressions and means "any Unicode scalar value" for `regex::Regex`
	/// expressions.
	pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut Builder {
		self.dot_matches_new_line = yes;
		self
	}

	/// Set the value for the greedy swap (`U`) flag.
	pub fn swap_greed(&mut self, yes: bool) -> &mut Builder {
		self.swap_greed = yes;
		self
	}

	/// Set the value for the ignore whitespace (`x`) flag.
	pub fn ignore_whitespace(&mut self, yes: bool) -> &mut Builder {
		self.ignore_whitespace = yes;
		self
	}

	/// Set the value for the Unicode (`u`) flag.
	pub fn unicode(&mut self, yes: bool) -> &mut Builder {
		self.unicode = yes;
		self
	}

	/// Set the approximate size limit of the compiled regular expression.
	///
	/// This roughly corresponds to the number of bytes occupied by a single
	/// compiled program. If the program exceeds this number, then a
	/// compilation error is returned.
	pub fn size_limit(&mut self, limit: usize) -> &mut Builder {
		self.size_limit = limit;
		self
	}

	/// Set the approximate size of the cache used by the DFA.
	///
	/// This roughly corresponds to the number of bytes that the DFA will
	/// use while searching.
	///
	/// Note that this is a *per thread* limit. There is no way to set a global
	/// limit. In particular, if a regex is used from multiple threads
	/// simulanteously, then each thread may use up to the number of bytes
	/// specified here.
	pub fn dfa_size_limit(&mut self, limit: usize) -> &mut Builder {
		self.dfa_size_limit = limit;
		self
	}
}
