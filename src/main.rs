use vortex_search::SearchEngine;

fn main() {
    println!("=======================================================");
    println!("   Vortex-Search: In-Memory Inverted Index BM25 Engine  ");
    println!("=======================================================");

    let mut engine = SearchEngine::new();

    println!("\n[1] Ingesting Technical Documentation Corpus...");
    let corpus = vec![
        (
            "LSM-Tree Storage Engines",
            "Log-Structured Merge-Trees write incoming mutations sequentially into an in-memory MemTable and write-ahead log. When full, MemTables flush to immutable disk SSTables, later merged through background compaction."
        ),
        (
            "Raft Consensus Protocol",
            "Raft is a consensus algorithm designed to be understandable. It divides state machine replication into leader election, log replication, and safety guarantees through randomized election timers."
        ),
        (
            "BM25 Ranking and Relevance",
            "The Okapi BM25 ranking function is a non-linear probabilistic retrieval framework that scores documents based on term frequencies and inverse document frequencies with length normalization."
        ),
        (
            "Concurrency and Lock-Free Queues",
            "Lock-free data structures utilize atomic primitives like compare-and-swap (CAS) to provide thread safety without mutex contention, enabling high-frequency event streaming."
        ),
        (
            "Vector Embeddings and Semantic Search",
            "Vector search represents unstructured text and images as high-dimensional dense embeddings, using Approximate Nearest Neighbor (ANN) graphs like HNSW for similarity queries."
        ),
    ];

    for (title, body) in &corpus {
        let id = engine.index_document(title, body);
        println!("    • Indexed Doc #{}: {}", id, title);
    }
    println!("    Total Corpus Size: {} documents", engine.total_documents());

    // Execute Sample Searches
    let queries = vec![
        "LSM-tree MemTable compaction SSTables",
        "probabilistic BM25 term frequency",
        "lock-free atomic concurrency",
        "distributed consensus leader election",
    ];

    println!("\n[2] Executing Ranked BM25 Full-Text Queries:");
    for q in queries {
        println!("\n-------------------------------------------------------");
        println!("Query: \"{}\"", q);
        let results = engine.search(q, 2);
        if results.is_empty() {
            println!("  No matches found.");
        } else {
            for (rank, res) in results.iter().enumerate() {
                println!("  #{} [{:.4} pts] {}", rank + 1, res.score, res.document.title);
                println!("     Snippet: \"{}\"", res.snippet);
            }
        }
    }

    println!("\n=======================================================");
    println!("Vortex-Search query processing completed successfully.");
}
