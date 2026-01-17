# Rust + React To-Do Application 🦀 ⚛️

A high-performance, full-stack To-Do application built with **Rust (Axum)**, **PostgreSQL**, and **React (TypeScript)**. This project demonstrates a clean "API-first" architecture with automated Swagger documentation and a minimalist dark-themed UI.

## 🚀 Features

* **High Performance Backend:** Built with Rust and Axum for blazing fast async request handling.
* **Type-Safe Database:** Uses `sqlx` for compile-time checked SQL queries against PostgreSQL.
* **Interactive API Docs:** Auto-generated Swagger/OpenAPI documentation via `utoipa`.
* **Modern Frontend:** React with TypeScript and Vite for a responsive user experience.
* **Dark Minimalist UI:** Custom CSS styling with a focus on usability and aesthetics.
* **Dockerized Database:** PostgreSQL runs in an isolated container for easy setup.

## FRONTEND:
![Frontend of the Application](image.png)

## SWAGGERS:
![swagger link](image-1.png)

## 🛠 Tech Stack

**Backend**

* **Language:** Rust
* **Framework:** Axum
* **Database:** PostgreSQL
* **ORM:** SQLx
* **Documentation:** Utoipa (Swagger UI)

**Frontend**

* **Framework:** React (Vite)
* **Language:** TypeScript
* **Styling:** CSS Modules (Custom Dark Theme)

---

## 📋 Prerequisites

Ensure you have the following installed:

* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* [Node.js (v18+) & npm](https://nodejs.org/)
* [Docker Desktop](https://www.docker.com/products/docker-desktop/)
* [DBeaver](https://dbeaver.io/) (Optional, for database management)

---

## ⚡ Getting Started

### 1. Database Setup (Docker)

Start the PostgreSQL container:

```bash
docker run --name todo_db \
  -e POSTGRES_USER=myuser \
  -e POSTGRES_PASSWORD=mypassword \
  -e POSTGRES_DB=todo_app \
  -p 5432:5432 \
  -d postgres

```

Initialize the database table. Connect via DBeaver (or CLI) and run:

```sql
CREATE TABLE todos (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);

```

### 2. Backend Setup (Rust)

1. Navigate to the `backend` folder:
```bash
cd backend

```


2. Create a `.env` file in the `backend` root:
```env
DATABASE_URL=postgres://myuser:mypassword@localhost:5432/todo_app

```


3. Run the server:
```bash
cargo run

```


* **Server:** `http://localhost:3000`
* **Swagger Docs:** `http://localhost:3000/swagger-ui`



### 3. Frontend Setup (React)

1. Open a new terminal and navigate to the `frontend` folder:
```bash
cd frontend

```


2. Install dependencies:
```bash
npm install

```


3. Start the development server:
```bash
npm run dev

```


* **App URL:** `http://localhost:5173`



---

## 📂 Project Structure

```text
rust-todo-app/
├── backend/
│   ├── Cargo.toml         # Rust dependencies
│   ├── .env               # Environment variables
│   └── src/
│       └── main.rs        # Backend logic & API handlers
│
└── frontend/
    ├── package.json       # Node dependencies
    ├── index.html         # HTML entry point
    └── src/
        ├── App.tsx        # React components & logic
        ├── App.css        # Minimalist dark styling
        └── main.tsx       # React DOM rendering

```

---

## 📖 API Documentation

The backend includes built-in Swagger UI. Once the backend is running, visit:
**[http://localhost:3000/swagger-ui](https://www.google.com/search?q=http://localhost:3000/swagger-ui)**

Available Endpoints:

* `GET /todos` - List all tasks
* `POST /todos` - Create a new task
* `PATCH /todos/{id}` - Toggle completion status
* `DELETE /todos/{id}` - Remove a task

---

## 🔧 Troubleshooting

**"Failed to parse manifest" / "No targets specified"**

* Ensure you are running `cargo run` from inside the `backend` folder, not `backend/src` or the root.
* Avoid running the project from your Trash or temporary folders. Move it to `Desktop` or `Documents`.

**Database Connection Refused**

* Ensure Docker Desktop is running.
* Check if the container is active: `docker ps`.
* Verify the `.env` credentials match your Docker command.

**CORS Errors**

* If the frontend cannot talk to the backend, ensure the `CorsLayer` is correctly applied in `main.rs`.

---

## 📜 License

This project is open source and available under the [MIT License](https://www.google.com/search?q=LICENSE).