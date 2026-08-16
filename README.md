# ⚙️ Rust Portfolio API

A GraphQL-based backend service for managing personal portfolio data — built with **Rust**, **Juniper**, and **MongoDB**.

This API is designed to power a personal portfolio website with flexible, structured access to profile information, social links, skills, and more — enabling dynamic frontend rendering and real-time updates through GraphQL.

🌐 Live API: [rust-portfolio-api.onrender.com](https://rust-portfolio-api.onrender.com)

---

## 🚀 Features

- 📌 **GraphQL API** (via `juniper`)
  - Fetch personal details, social media links, skills, and more
- 🗃️ **MongoDB** Integration
  - Stores user data and retrieves it via GraphQL resolvers
- 🛡️ **CORS Support**
  - Enables cross-origin requests from your frontend portfolio
- 🐳 **Docker & Helm Support**
  - Ready for containerized deployment and orchestration

---

## 🧱 Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/)
- **GraphQL Library:** [Juniper](https://github.com/graphql-rust/juniper)
- **Web Framework:** Axum
- **Database:** MongoDB
- **Containerization:** Docker
- **Deployment Ready:** Render, Helm (Kubernetes)

---

## 🗂️ Project Structure

```plaintext
rust-portfolio-api/
├── src/               # Rust source files (resolvers, schemas, GraphQL handlers)
├── portfolioapi/      # Helm chart configs for deployment
├── Dockerfile         # Docker container config
├── DockerfileHelm     # Alternative Dockerfile for Helm deploy
├── Cargo.toml         # Rust dependencies and package metadata
```

## 🛠️ Getting Started

### 1. Clone the Repo

```bash
git clone https://github.com/johnmichealacera/rust-portfolio-api.git
cd rust-portfolio-api
```

### 2. Setup Environment Variables
Create a .env file or export:

```bash
MONGO_DB_URI=mongodb+srv://<your_connection_string>
USER_EMAIL=youremail@example.com
```

### 3. Run Locally
```bash
cargo run
```

### 4. Test GraphQL
Open your browser or use a tool like Postman to visit:

```bash
http://localhost:8000/graphql
```

### 🐳 Docker
```bash
docker build -t rust-portfolio-api .
docker run -p 8000:8000 rust-portfolio-api
```

## 🧪 Example GraphQL Query

```bash
query {
  stories {
    slug
    title
    excerpt
    status
    chapters {
      id
      number
      title
      scenes {
        id
        video
        image
        imageAlt
        blocks {
          blockType
          value
          speaker
        }
      }
    }
  }
}
```

Seed public stories into MongoDB:

```bash
cargo run --bin seed_stories
```

## 📄 License
This project is licensed under the MIT License. See LICENSE for more information.

## 👨‍💻 Author

John Micheal Acera

- 🌐 [Portfolio Website](https://johnmichealacera.vercel.app/)
- 🐙 [GitHub](https://github.com/johnmichealacera/)
 
 > **This is a backend foundation for my portfolio, built using a language I admire for its performance, safety, and elegance — Rust**
