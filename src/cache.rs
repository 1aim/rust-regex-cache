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

use std::ops::{Deref, DerefMut};

use regex::{Regex, RegexBuilder, Error};
use lru::LruCache;

/// An LRU cache for regular expressions.
pub struct RegexCache(LruCache<String, Regex>);

impl RegexCache {
	/// Create a new LRU cache with the given size limit.
	pub fn new(capacity: usize) -> RegexCache {
		RegexCache(LruCache::new(capacity))
	}

	/// Save the given regular expression in the cache.
	///
	/// # Example
	///
	/// ```
	/// # use regex_cache::{Regex, RegexCache};
	/// let mut cache = RegexCache::new(100);
	/// let     re    = Regex::new(r"^\d+$").unwrap();
	///
	/// // By saving the previously created regular expression further calls to
	/// // `compile` won't actually compile the regular expression.
	/// cache.save(re);
	///
	/// assert!(cache.compile(r"^\d+$").unwrap().is_match("1234"));
	/// assert!(!cache.compile(r"^\d+$").unwrap().is_match("abcd"));
	/// ```
	pub fn save(&mut self, re: Regex) -> &Regex {
		let source = re.as_str().to_owned();

		if !self.0.contains_key(re.as_str()) {
			self.insert(source.clone(), re);
		}

		self.0.get_mut(&source).unwrap()
	}

	/// Create a new regular expression in the cache.
	///
	/// # Example
	///
	/// ```
	/// # use regex_cache::RegexCache;
	/// let mut cache = RegexCache::new(100);
	///
	/// assert!(cache.compile(r"^\d+$").unwrap().is_match("1234"));
	/// assert!(!cache.compile(r"^\d+$").unwrap().is_match("abcd"));
	/// ```
	pub fn compile(&mut self, source: &str) -> Result<&Regex, Error> {
		if !self.0.contains_key(source) {
			self.0.insert(source.into(), Regex::new(source)?);
		}

		Ok(self.0.get_mut(source).unwrap())
	}

	/// Configure a new regular expression.
	///
	/// # Example
	///
	/// ```
	/// # use regex_cache::RegexCache;
	/// let mut cache = RegexCache::new(100);
	///
	/// assert!(cache.configure(r"abc", |b| b.case_insensitive(true)).unwrap()
	/// 	.is_match("ABC"));
	///
	/// assert!(!cache.configure(r"abc", |b| b.case_insensitive(true)).unwrap()
	/// 	.is_match("123"));
	/// ```
	pub fn configure<F>(&mut self, source: &str, f: F) -> Result<&Regex, Error>
		where F: FnOnce(&mut RegexBuilder) -> &mut RegexBuilder
	{
		if !self.0.contains_key(source) {
			self.0.insert(source.into(), f(&mut RegexBuilder::new(source)).build()?);
		}

		Ok(self.0.get_mut(source).unwrap())
	}
}

impl Deref for RegexCache {
	type Target = LruCache<String, Regex>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for RegexCache {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[cfg(test)]
mod test {
	use cache::RegexCache;

	#[test]
	fn respects_limit() {
		let mut cache = RegexCache::new(2);

		cache.compile("[01]2").unwrap();
		cache.compile("[21]0").unwrap();

		assert_eq!(cache.len(), 2);
		cache.compile("[21]3").unwrap();
		assert_eq!(cache.len(), 2);
	}
}
