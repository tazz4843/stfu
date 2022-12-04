//! Filtering words from lists.

/// Returns true if the phrase contains a word from the list.
///
/// This function is short-circuiting: the first word found will return true.
///
/// Note that the list must be lowercase, otherwise unreliable results may occur.
///
/// Results are not guaranteed to be returned in the order in which they appear in the phrase.
///
/// # Examples
/// ```
/// use stfu::word_lists::category::{RELIGIOUS_OFFENSE, SEXUAL_ANATOMY_SEXUAL_ACTS};
/// # fn main() {
/// assert_eq!(stfu::filter::filter_string("Go to hell you idiot!", &RELIGIOUS_OFFENSE), Some("hell"));
/// assert_eq!(stfu::filter::filter_string("Mary had a little fucking lamb", &SEXUAL_ANATOMY_SEXUAL_ACTS), Some("fucking"));
/// # }
/// ```
pub fn filter_string<'a, 'b, T: AsRef<str>>(s: &'a str, list: &'b [T]) -> Option<&'b str> {
    let lower = s.to_lowercase();

    for list_word in list {
        let list_word = list_word.as_ref();

        // to avoid the scunthorpe problem, if a word is found, we check if it is a word boundary
        // (i.e. not part of a larger word)
        if let Some(idx) = lower.find(list_word) {
            let before = idx.checked_sub(1);
            let after = idx.checked_add(list_word.len());

            let start = before.map_or(true, |i| {
                lower.chars().nth(i).map_or(true, |c| !c.is_alphabetic())
            });
            let end = after.map_or(true, |i| {
                lower.chars().nth(i).map_or(true, |c| !c.is_alphabetic())
            });

            if start && end {
                return Some(list_word);
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    // Do not continue reading if you are offended easily.

    use super::*;

    /// Tests the `filter_string` function.
    #[test]
    fn check_filter_string() {
        assert_eq!(filter_string("hello world", &["hello"]), Some("hello"));

        assert_eq!(
            filter_string(
                "mary had a little fucking cunt of a lamb",
                &crate::word_lists::category::SEXUAL_ANATOMY_SEXUAL_ACTS
            ),
            Some("cunt")
        );
        assert_eq!(
            filter_string(
                "some shithead jackass came by and asked us if we had any fags",
                &crate::word_lists::category::SEXUAL_ORIENTATION_GENDER
            ),
            Some("fags")
        );
        assert_eq!(
            filter_string(
                "scunthorpe is a nice place",
                &crate::word_lists::category::SEXUAL_ANATOMY_SEXUAL_ACTS,
            ),
            None
        );
        assert_eq!(
            filter_string(
                "penistone has many nice markets",
                &crate::word_lists::category::SEXUAL_ANATOMY_SEXUAL_ACTS
            ),
            None
        );
    }
}
