use crate::index::{Document, InvertedIndex};
use crate::ranking::BM25Scorer;
use crate::token::Tokenizer;

pub struct SearchResult<'a> {
    pub document: &'a Document,
    pub score: f64,
    pub snippet: String,
}

pub struct SearchEngine {
    index: InvertedIndex,
    tokenizer: Tokenizer,
    scorer: BM25Scorer,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: InvertedIndex::new(),
            tokenizer: Tokenizer::default(),
            scorer: BM25Scorer::default(),
        }
    }

    pub fn index_document(&mut self, title: &str, body: &str) -> usize {
        self.index.add_document(title, body)
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Vec<SearchResult> {
        let query_tokens = self.tokenizer.tokenize(query_str);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        let ranked = self.scorer.score_query(&self.index, &query_tokens);

        ranked
            .into_iter()
            .take(limit)
            .filter_map(|(doc_id, score)| {
                self.index.documents.get(&doc_id).map(|doc| {
                    let snippet = self.generate_snippet(&doc.body, &query_tokens);
                    SearchResult {
                        document: doc,
                        score,
                        snippet,
                    }
                })
            })
            .collect()
    }

    fn generate_snippet(&self, text: &str, query_terms: &[String]) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let clean = word.to_lowercase();
            if query_terms.iter().any(|term| clean.contains(term)) {
                let start = i.saturating_sub(4);
                let end = (i + 8).min(words.len());
                let snippet = words[start..end].join(" ");
                return format!("... {} ...", snippet);
            }
        }
        words.iter().take(12).copied().collect::<Vec<&str>>().join(" ")
    }

    pub fn total_documents(&self) -> usize {
        self.index.document_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_relevance() {
        let mut engine = SearchEngine::new();
        engine.index_document(
            "Rust Concurrency",
            "Fearless concurrency in Rust prevents data races at compile time with channels and mutexes.",
        );
        engine.index_document(
            "Go Goroutines",
            "Go provides lightweight threads called goroutines and channels for CSP style concurrency.",
        );
        engine.index_document(
            "Python AsyncIO",
            "Python uses an event loop and async await syntax for single threaded asynchronous execution.",
        );

        let results = engine.search("Rust concurrency compile time", 3);
        assert!(!results.is_empty());
        assert_eq!(results[0].document.title, "Rust Concurrency");
        assert!(results[0].score > 0.0);
    }
}
