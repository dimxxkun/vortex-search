use crate::index::InvertedIndex;

pub struct BM25Scorer {
    k1: f64,
    b: f64,
}

impl Default for BM25Scorer {
    fn default() -> Self {
        Self { k1: 1.2, b: 0.75 }
    }
}

impl BM25Scorer {
    pub fn new(k1: f64, b: f64) -> Self {
        Self { k1, b }
    }

    /// Computes BM25 score for a specific term and document posting.
    pub fn score_term(
        &self,
        term_freq: usize,
        doc_length: usize,
        avg_doc_length: f64,
        doc_count: usize,
        docs_containing_term: usize,
    ) -> f64 {
        if doc_count == 0 || docs_containing_term == 0 {
            return 0.0;
        }

        // Robertson-Spärck Jones IDF formula
        let n = doc_count as f64;
        let df = docs_containing_term as f64;
        let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();

        let tf = term_freq as f64;
        let len_norm = 1.0 - self.b + self.b * (doc_length as f64 / avg_doc_length.max(1.0));
        let num = tf * (self.k1 + 1.0);
        let denom = tf + self.k1 * len_norm;

        idf * (num / denom)
    }

    pub fn score_query(&self, index: &InvertedIndex, query_terms: &[String]) -> Vec<(usize, f64)> {
        use std::collections::HashMap;

        let total_docs = index.document_count();
        let avg_len = index.avg_document_length();
        let mut scores: HashMap<usize, f64> = HashMap::new();

        for term in query_terms {
            if let Some(postings) = index.get_postings(term) {
                let df = postings.len();
                for posting in postings {
                    if let Some(doc) = index.documents.get(&posting.doc_id) {
                        let term_score = self.score_term(
                            posting.term_frequency,
                            doc.token_count,
                            avg_len,
                            total_docs,
                            df,
                        );
                        *scores.entry(posting.doc_id).or_insert(0.0) += term_score;
                    }
                }
            }
        }

        let mut ranked: Vec<(usize, f64)> = scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }
}
