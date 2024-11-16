use reqwest::Client;
use serde::{Deserialize};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use lopdf::Document;

// Struct for PDF Documents (simplified for this example)
#[derive(Debug, Clone)]
struct DocumentStruct {
    id: String,
    content: String,
}

// Embedding Response
#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

// Azure OpenAI API Client
struct AzureClient;

impl AzureClient {
    async fn get_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        let client = Client::new();
        let url = "https://<YOUR-AZURE-ENDPOINT>/openai/deployments/<YOUR-DEPLOYMENT-ID>/embeddings?api-version=2023-05-15"; // Replace with your endpoint
        let body = json!({
            "input": [text]
        });

        let response = client
            .post(url)
            .header("Content-Type", "application/json")
            .header("api-key", "<YOUR-API-KEY>") // Replace with your API key
            .json(&body)
            .send()
            .await?;

        let embedding_response: EmbeddingResponse = response.json().await?;
        Ok(embedding_response.data[0].embedding.clone())
    }
}

// In-Memory Vector Database (simplified Chroma-like)
struct VectorDB {
    store: Arc<Mutex<HashMap<String, (Vec<f32>, DocumentStruct)>>>,
}

impl VectorDB {
    fn new() -> Self {
        VectorDB {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_document(&self, id: String, embedding: Vec<f32>, doc: DocumentStruct) {
        let mut store = self.store.lock().unwrap();
        store.insert(id, (embedding, doc));
    }

    fn query(&self, query_embedding: Vec<f32>, top_k: usize) -> Vec<DocumentStruct> {
        let store = self.store.lock().unwrap();
        let mut results: Vec<(f32, DocumentStruct)> = store
            .values()
            .map(|(embedding, doc)| {
                let similarity = cosine_similarity(&query_embedding, embedding);
                (similarity, doc.clone())
            })
            .collect();

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        results.into_iter().take(top_k).map(|(_, doc)| doc).collect()
    }
}

// Cosine Similarity Calculation
fn cosine_similarity(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
    let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(x, y)| x * y).sum();
    let magnitude1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (magnitude1 * magnitude2)
}

// GPT-3 Model Response
#[derive(Deserialize)]
struct GPTResponse {
    choices: Vec<GPTChoice>,
}

#[derive(Deserialize)]
struct GPTChoice {
    text: String,
}

// Function to Query GPT
async fn query_gpt(prompt: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://<YOUR-AZURE-ENDPOINT>/openai/deployments/<YOUR-DEPLOYMENT-ID>/completions?api-version=2023-05-15"; // Replace with your endpoint
    let body = json!({
        "prompt": prompt,
        "max_tokens": 500,
        "temperature": 0.7,
        "top_p": 0.9,
        "frequency_penalty": 0,
        "presence_penalty": 0
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("api-key", "<YOUR-API-KEY>") // Replace with your API key
        .json(&body)
        .send()
        .await?;

    let gpt_response: GPTResponse = response.json().await?;
    Ok(gpt_response.choices[0].text.trim().to_string())
}

// Function to read PDF content
fn read_pdf(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let doc = Document::load_mem(&buffer)?;
    let mut content = String::new();

    // Get total number of pages
    let pages = doc.get_pages();
    println!("Total pages: {}", pages.len()); // Log the number of pages

    for (page_number, _) in pages {
        let page_content = doc.extract_text(&[page_number])?; // Extract text from existing pages
        content.push_str(&page_content);
    }

    Ok(content)
}

// Main Function for Document Query and GPT Answer
async fn answer_query(
    user_query: &str,
    vector_db: &VectorDB,
    azure_client: &AzureClient,
) -> Result<String, Box<dyn Error>> {
    // Step 1: Embed the user's query
    let query_embedding = azure_client.get_embedding(user_query).await?;

    // Step 2: Query VectorDB for similar documents
    let top_documents = vector_db.query(query_embedding, 5);

    // Step 3: Combine the context from the documents
    let context: String = top_documents
        .into_iter()
        .map(|doc| doc.content)
        .collect::<Vec<String>>()
        .join(" ");

    // Step 4: Query GPT for the answer based on the context
    let prompt = format!("Context: {}\n\nQuestion: {}\n\nAnswer:", context, user_query);
    let gpt_response = query_gpt(&prompt).await?;

    Ok(gpt_response)
}

// Example usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create the in-memory vector database
    let vector_db = VectorDB::new();

    // Read the resume PDF
    let pdf_path = "src/PDF/Fahads_Resume.pdf";  // Correct path
    let resume_content = read_pdf(pdf_path)?;

    // Example Document
    let doc = DocumentStruct {
        id: "doc_1".to_string(),
        content: resume_content,
    };

    // Initialize Azure API Client (no credentials here)
    let azure_client = AzureClient;

    // Embed the document content and add to the database
    let doc_embedding = azure_client.get_embedding(&doc.content).await?;
    vector_db.add_document("doc_1".to_string(), doc_embedding, doc);

    // Answer user query
    let user_query = "What experience is listed in the resume?";
    let answer = answer_query(user_query, &vector_db, &azure_client).await?;

    println!("Answer: {}", answer);
    Ok(())
}
