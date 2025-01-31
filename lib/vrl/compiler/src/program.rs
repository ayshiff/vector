use crate::Expression;
use std::iter::IntoIterator;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Program {
    pub(crate) expressions: Vec<Box<dyn Expression>>,
    pub(crate) fallible: bool,
}

impl Program {
    /// Returns whether the compiled program can fail at runtime.
    ///
    /// A program can only fail at runtime if the fallible-function-call
    /// (`foo!()`) is used within the source.
    pub fn is_fallible(&self) -> bool {
        self.fallible
    }
}

impl IntoIterator for Program {
    type Item = Box<dyn Expression>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.expressions.into_iter()
    }
}

impl Deref for Program {
    type Target = [Box<dyn Expression>];

    fn deref(&self) -> &Self::Target {
        &self.expressions
    }
}
