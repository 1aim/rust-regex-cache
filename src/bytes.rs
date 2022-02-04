pub use regex::bytes::{Regex, RegexBuilder};

mod cache;
pub use cache::{CachedRegex, CachedRegexBuilder, RegexCache};

mod lazy;
pub use lazy::{LazyRegex, LazyRegexBuilder};

mod options;
