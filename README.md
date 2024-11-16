🦀 Rust-Based Resume Analyzer with Azure OpenAI 💼📄
Welcome to the Resume Analyzer, a cutting-edge tool built in Rust 🦀, powered by Azure OpenAI Services ☁️🤖, designed to streamline resume analysis and answer queries efficiently for HR teams.

🚀 Project Highlights
🛠 Core Features:
Extract Resume Content: Effortlessly extracts text 📄 from PDF resumes using the powerful lopdf crate.
Generate Embeddings: Converts text into high-dimensional embeddings 🔢 using the text-embedding-ada-002 model from Azure OpenAI ☁️🤖.
In-Memory Vector Database: Stores embeddings in a custom-designed in-memory vector database for lightning-fast ⚡ similarity searches.
Cosine Similarity Matching: Leverages cosine similarity 📐 to rank resumes based on relevance to user queries.
Context-Aware Responses: Integrates GPT 🤖 to provide intelligent, context-aware answers to HR queries by combining matched resumes and user input.

⚙️ Tech Stack
Rust 🦀: High-performance language for building the backend.
Azure OpenAI ☁️🤖: Used for embeddings and GPT-based querying.
lopdf: Crate for PDF parsing and text extraction.
Cosine Similarity 📐: For relevance-based ranking of resumes.
