//! Utility types.

/// An alternative filter type that allows general addition of words.
///
/// # Usage
/// ```
/// use stfu::word_lists::category::SEXUAL_ANATOMY_SEXUAL_ACTS;
/// use stfu::types::OwnedFilter;
/// # fn main() {
/// let mut filter = OwnedFilter::default();
/// filter.add_slice(&SEXUAL_ANATOMY_SEXUAL_ACTS);
/// assert_eq!(filter.filter_string("hello world"), None);
/// assert_eq!(filter.filter_string("Mary had a little fucking lamb"), Some("fucking"));
/// # }
pub struct OwnedFilter(Vec<String>);

impl OwnedFilter {
    /// Creates a new `OwnedFilter` with the given list of words.
    pub fn new(list: Vec<String>) -> Self {
        Self(list)
    }

    /// Creates a empty `OwnedFilter`.
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Adds a word to the filter.
    pub fn add_word(&mut self, word: String) {
        self.0.push(word);
    }

    /// Adds a Vec of words to the filter.
    pub fn add_vec(&mut self, vec: Vec<String>) {
        self.0.extend(vec);
    }

    /// Adds a slice of words to the filter.
    pub fn add_slice<T: AsRef<str>>(&mut self, slice: &[T]) {
        self.0.extend(slice.iter().map(|s| s.as_ref().to_string()));
    }

    /// Returns the list of words.
    pub fn word_list(&self) -> &[String] {
        &self.0
    }

    /// Removes a word from the filter.
    ///
    /// Warning: this is an expensive operation. Filter the words you add to the filter, instead.
    pub fn remove_word(&mut self, word: &str) {
        self.0.retain(|s| s != word);
    }

    /// Returns true if the phrase contains a word in the filter.
    ///
    /// This simply calls [crate::filter::filter_string], see its docs for more info.
    pub fn filter_string(&self, s: &str) -> Option<&str> {
        crate::filter::filter_string(s, &self.0)
    }
}

impl AsRef<[String]> for OwnedFilter {
    fn as_ref(&self) -> &[String] {
        self.word_list()
    }
}

impl Default for OwnedFilter {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_owned_filter() {
        let mut filter = OwnedFilter::empty();
        filter.add_word("hello".to_string());
        filter.add_vec(vec![
            "world".to_string(),
            "this".to_string(),
            "is".to_string(),
        ]);
        filter.add_slice(&["a", "test"]);
        assert_eq!(
            filter.word_list(),
            &["hello", "world", "this", "is", "a", "test"]
        );
    }
}
