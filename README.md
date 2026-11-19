# Vortex-Search

[![License: Noncommercial](https://img.shields.io/badge/License-Noncommercial-blue.svg)](LICENSE)

**Vortex-Search** is a fast, embeddable in-memory full-text search engine implemented in pure Rust. It features tokenization with stop-word removal, positional inverted index construction, and the Okapi BM25 probabilistic relevance ranking algorithm.

---

## Architecture

```
 Corpus Input
     │
     ▼
[ Tokenizer ] ──(Stop-word filtering & Lowercasing)
     │
     ▼
[ Inverted Index ] ──(Term -> Posting Lists: [doc_id, freq, positions])
     │
     ▼
[ BM25 Relevance Scorer ] ──(Term Saturation + Document Length Normalization)
     │
     ▼
[ Query Evaluator & Snippet Generator ] ──> Ranked Search Results
```

## BM25 Scoring Equation

$$\text{Score}(D, Q) = \sum_{i=1}^{N} \text{IDF}(q_i) \cdot \frac{f(q_i, D) \cdot (k_1 + 1)}{f(q_i, D) + k_1 \cdot \left(1 - b + b \cdot \frac{|D|}{\text{avgdl}}\right)}$$

Where:
- $k_1 = 1.2$: Controls term frequency saturation limits.
- $b = 0.75$: Controls document length penalization degree.
- $\text{IDF}(q_i) = \ln\left( \frac{N - n(q_i) + 0.5}{n(q_i) + 0.5} + 1 \right)$: Robertson-Spärck Jones Inverse Document Frequency.

## Project Structure

```
vortex-search/
├── Cargo.toml
├── README.md
├── .gitignore
└── src/
    ├── lib.rs
    ├── main.rs
    ├── token/
    │   ├── mod.rs
    │   └── tokenizer.rs
    ├── index/
    │   ├── mod.rs
    │   └── inverted_index.rs
    ├── ranking/
    │   ├── mod.rs
    │   └── bm25.rs
    └── query/
        ├── mod.rs
        └── engine.rs
```

## Quick Start

### Run the Demo:
```bash
cargo run
```

### Run Tests:
```bash
cargo test
```

## Sample Output
```text
Query: "LSM-tree MemTable compaction SSTables"
  #1 [1.8412 pts] LSM-Tree Storage Engines
     Snippet: "... Log-Structured Merge-Trees write incoming mutations sequentially into an in-memory MemTable and write-ahead log. When full, MemTables flush ..."
```

## License
[Noncommercial license](LICENSE) © [dimxxkun](https://github.com/dimxxkun)

## Commercial permission

Original material in this repository is available under the [Noncommercial Use and Commercial Permission License](LICENSE). **Noncommercial use is free. Commercial use requires prior written permission and a separate written agreement specifying the scope and agreed fee.** Contact [work.abdulfatai@gmail.com](mailto:work.abdulfatai@gmail.com) with the repository name and intended use.

Portfolio review and recruitment evaluation are permitted without a fee. Third-party components retain their own licenses. Permissions already granted for earlier versions remain valid; the new terms do not revoke those grants. These are custom source-available terms, not an OSI-approved open-source license.
