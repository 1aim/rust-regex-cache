regex-cache [![Crates.io](https://img.shields.io/crates/v/regex-cache.svg)](https://crates.io/crates/regex-cache) ![regex-cache](https://docs.rs/regex-cache/badge.svg) ![MIT](http://img.shields.io/badge/license-MIT-blue.svg) [![Build Status](https://travis-ci.org/1aim/rust-regex-cache.svg?branch=master)](https://travis-ci.org/1aim/rust-regex-cache)
===========
This crate provides a library for caching or lazily creating regular
expressions.

Lazy regular expressions are backed by Thread Local Storage, while the
regular expression cache is backed by a Least Recently Used cache.

Why not `lazy_static!`?
-----------------------
`lazy_static!` is great but it's only usable when you know what your regular
expression is.

In some cases you're loading an insane number of regular expressions (looking
at you libphonenumber) that you might eventually use but don't actually want to
store as `String` since that makes you lose type information and ease of use.

When to use `LazyRegex`
-----------------------
When you have a lot of regular expressions you don't want to use instantly and
you don't care about having them still allocated after being used.

Since they're backed by Thread Local Storage you also have to keep in mind that
they will be compiled again every time the `LazyRegex` is accessed in a
different thread.

If you're passing around the regular expression to newly created threads you
might want to use a locked `RegexCache` instead.

When to use `RegexCache`
------------------------
When you want to limit the number of active regular expressions, the
`RegexCache` only keeps around a specified number of actively used regular
expressions.

Since it's an LRU cache having a small limit and using a different regular
expression every time ends up wasting memory and time for nothing.
