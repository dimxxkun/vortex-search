use std::collections::HashMap;
use crate::token::Tokenizer;

#[derive(Debug, Clone)]
pub struct Posting {
    pub doc_id: usize,
    pub term_frequency: usize,
    pub positions: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: usize,
    pub title: String,
    pub body: String,
    pub token_count: usize,
}

pub struct InvertedIndex {
    pub documents: HashMap<usize, Document>,
    pub index: HashMap<String, Vec<Posting>>,
    pub total_tokens: usize,
    tokenizer: Tokenizer,
    next_id: usize,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            index: HashMap::new(),
            total_tokens: 0,
            tokenizer: Tokenizer::default(),
            next_id: 1,
        }
    }

    pub fn add_document(&mut self, title: &str, body: &str) -> usize {
        let doc_id = self.next_id;
        self.next_id += 1;

        let tokens = self.tokenizer.tokenize_with_positions(body);
        let token_count = tokens.len();
        self.total_tokens += token_count;

        let mut term_positions: HashMap<String, Vec<usize>> = HashMap::new();
        for (term, pos) in tokens {
            term_positions.entry(term).or_default().push(pos);
        }

        for (term, positions) in term_positions {
            let posting = Posting {
                doc_id,
                term_frequency: positions.len(),
                positions,
            };
            self.index.entry(term).or_default().push(posting);
        }

        self.documents.insert(
            doc_id,
            Document {
                id: doc_id,
                title: title.to_string(),
                body: body.to_string(),
                token_count,
            },
        );

        doc_id
    }

    pub fn get_postings(&self, term: &str) -> Option<&Vec<Posting>> {
        self.index.get(term)
    }

    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn avg_document_length(&self) -> f64 {
        if self.documents.is_empty() {
            0.0
        } else {
            self.total_tokens as f64 / self.documents.len() as f64
        }
    }
}
