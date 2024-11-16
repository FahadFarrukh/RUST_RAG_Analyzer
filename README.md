<h1 style="text-align: center;">🦀 Rust-Based Resume Analyzer with Azure OpenAI 💼📄</h1>

<p style="text-align: center;">
    Welcome to the Resume Analyzer, a cutting-edge tool built in Rust 🦀, powered by Azure OpenAI Services ☁️🤖.  
    It is designed to streamline resume analysis and answer queries efficiently for HR teams.
</p>

<br>

<h2 style="text-align: center;">🚀 Project Highlights</h2>

<h3 style="text-align: left;">🛠 Core Features:</h3>
<ul style="line-height: 1.8;">
    <li>
        <strong>Extract Resume Content:</strong> Effortlessly extracts text 📄 from PDF resumes using the powerful 
        <code>lopdf</code> crate.
    </li>
    <li>
        <strong>Generate Embeddings:</strong> Converts text into high-dimensional embeddings 🔢 using the 
        text-embedding-ada-002 model from Azure OpenAI ☁️🤖.
    </li>
    <li>
        <strong>In-Memory Vector Database:</strong> Stores embeddings in a custom-designed in-memory vector database for 
        lightning-fast ⚡ similarity searches.
    </li>
    <li>
        <strong>Cosine Similarity Matching:</strong> Leverages cosine similarity 📐 to rank resumes based on relevance to 
        user queries.
    </li>
    <li>
        <strong>Context-Aware Responses:</strong> Integrates GPT 🤖 to provide intelligent, context-aware answers to HR 
        queries by combining matched resumes and user input.
    </li>
</ul>

<br>

<h2 style="text-align: center;">⚙️ Tech Stack</h2>

<ul style="line-height: 1.8;">
    <li><strong>Rust 🦀:</strong> High-performance language for building the backend.</li>
    <li><strong>Azure OpenAI ☁️🤖:</strong> Used for embeddings and GPT-based querying.</li>
    <li><strong>lopdf:</strong> Crate for PDF parsing and text extraction.</li>
    <li><strong>Cosine Similarity 📐:</strong> For relevance-based ranking of resumes.</li>
</ul>
