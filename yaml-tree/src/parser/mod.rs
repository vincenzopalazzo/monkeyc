/// Main Yaml parser crate
///
/// author: https://github.com/vincenzopalazzo
use std::vec::Vec;

use crate::scanner::tokens;

pub mod parser;

/// Super type of Yaml node
/// TODO: we can use a enum?
pub struct YamlNode {}

/// Interface Yaml Interface
trait YamlParser<T> {
    fn parse(&mut self, tokens: &Vec<tokens::YamlToken>) -> &Vec<T>;
}
