use std::fmt::Display;
use std::hash::Hash;

pub trait Category:
    Copy + Clone + Display + PartialEq + Eq + Hash + Default
{
}
