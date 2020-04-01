#![macro_use]

use std::borrow::Borrow;
use regex::Regex;

pub mod either;
pub mod jwt;

mod url_regex;
pub type UrlRegex = url_regex::UrlRegex;

mod unique_vec;
pub type UniqueVec<T> = unique_vec::UniqueVec<T>;