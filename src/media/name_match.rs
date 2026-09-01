// The API and our name tables disagree on case, punctuation and trailing
// category words, so both sides are reduced to the same shape before
// they are compared.
const TRAILING_WORDS: [&str; 4] = ["talisman", "cookbook", "bell bearing", "whetblade"];

pub fn normalise(name: &str) -> String {
    let mut text = name.to_lowercase();

    // Drop a "+N" upgrade suffix.
    if let Some(pos) = text.rfind('+') {
        if text[pos + 1..].trim().chars().all(|c| c.is_ascii_digit())
            && !text[pos + 1..].trim().is_empty()
        {
            text.truncate(pos);
        }
    }

    // Punctuation becomes a space so "two-headed" and "two headed" agree.
    let spaced: String = text
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();

    let mut words: Vec<&str> = spaced.split_whitespace().collect();

    for trailing in TRAILING_WORDS {
        let parts: Vec<&str> = trailing.split(' ').collect();
        if words.len() > parts.len() && words.ends_with(&parts) {
            words.truncate(words.len() - parts.len());
            break;
        }
    }

    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::normalise;

    #[test]
    fn ignores_case_spacing_and_punctuation() {
        assert_eq!(normalise("Hand Axe"), normalise("hand  axe"));
        assert_eq!(normalise("Shard of Alexander"), normalise("Shard Of Alexander"));
        assert_eq!(normalise("Two-Headed Turtle"), normalise("Two Headed Turtle"));
    }

    #[test]
    fn drops_upgrade_suffix() {
        assert_eq!(normalise("Hand Axe +5"), normalise("Hand Axe"));
        assert_eq!(normalise("Hand Axe +25"), normalise("Hand Axe"));
    }

    #[test]
    fn drops_trailing_category_word() {
        assert_eq!(normalise("Crimson Seed Talisman"), normalise("Crimson Seed"));
    }

    #[test]
    fn keeps_distinct_names_distinct() {
        assert_ne!(normalise("Hand Axe"), normalise("Great Axe"));
    }
}
