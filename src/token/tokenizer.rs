use std::collections::HashSet;

pub struct Tokenizer {
    stop_words: HashSet<String>,
}

impl Default for Tokenizer {
    fn default() -> Self {
        let stop_words_list = [
            "a", "about", "an", "and", "are", "as", "at", "be", "by", "for",
            "from", "how", "i", "in", "is", "it", "of", "on", "or", "that",
            "the", "this", "to", "was", "what", "when", "where", "who", "will", "with"
        ];
        let mut stop_words = HashSet::new();
        for word in stop_words_list {
            stop_words.insert(word.to_string());
        }
        Self { stop_words }
    }
}

impl Tokenizer {
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        text.split(|c: char| !c.is_alphanumeric())
            .filter(|token| !token.is_empty())
            .map(|token| token.to_lowercase())
            .filter(|token| !self.stop_words.contains(token) && token.len() > 1)
            .collect()
    }

    pub fn tokenize_with_positions(&self, text: &str) -> Vec<(String, usize)> {
        let mut tokens = Vec::new();
        let mut pos = 0;
        for raw in text.split(|c: char| !c.is_alphanumeric()) {
            if !raw.is_empty() {
                let lower = raw.to_lowercase();
                if !self.stop_words.contains(&lower) && lower.len() > 1 {
                    tokens.push((lower, pos));
                }
                pos += 1;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let tokenizer = Tokenizer::default();
        let tokens = tokenizer.tokenize("The quick brown fox jumps over the lazy dog!");
        assert!(tokens.contains(&"quick".to_string()));
        assert!(tokens.contains(&"brown".to_string()));
        assert!(!tokens.contains(&"the".to_string())); // Stopword removed
    }
}
