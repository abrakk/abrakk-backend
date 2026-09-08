# EduKit

> An open-source platform for discovering, creating, and sharing educational learning activities and printable learning kits.

EduKit connects teachers, parents, educators, and content creators through an open platform where quality learning resources can be collaboratively developed and shared. Built for accessibility, cultural relevance, and community-driven growth.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)](./CONTRIBUTING.md)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

---

## Why EduKit?

Quality educational resources are not always easy to access. Teachers and parents often spend significant time searching for suitable learning activities, adapting resources to different age groups, and creating materials from scratch.

EduKit provides an open platform where educational resources can be discovered, created, improved, and shared by the community.

**Our goal:** Make quality learning resources more accessible through open collaboration.

---

## Features

- 🔎 Browse and search educational learning kits
- 📚 Filter by age group, subject, difficulty, and language
- 🧩 Explore learning activities within each kit
- 📥 Download printable educational materials
- ✍️ Create and share your own educational kits
- 👩‍🏫 Dashboard for teachers and content creators
- 🌍 Support for culturally relevant and multi-language content
- 📱 Fully responsive across all devices
- ♿ Accessibility-first design

---

## Architecture

EduKit uses a decoupled architecture with a Next.js frontend and a Rust backend.

```
┌──────────────────────┐
│   Next.js Frontend   │
│  React + TypeScript  │
│     Tailwind CSS     │
└──────────┬───────────┘
           │
      REST API / JSON
           │
           ▼
┌──────────────────────┐
│    Rust Backend      │
│   Axum + Tokio       │
│   SQLx + PostgreSQL  │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│     PostgreSQL       │
│      Database        │
└──────────────────────┘
```

The frontend and backend are independently structured, so contributors can work on the UI, API, database layer, tests, docs, or DevOps without unnecessary coupling.

---

## Tech Stack

| Layer        | Technology                          |
|--------------|-------------------------------------|
| Frontend     | Next.js 14, React, TypeScript, Tailwind CSS |
| Backend      | Rust, Axum, Tokio                   |
| Database     | PostgreSQL                          |
| DB Access    | SQLx                                |
| Auth         | JWT (JSON Web Tokens)               |
| Serialization| Serde                               |
| Passwords    | Argon2                              |
| Logging      | tracing                             |
| Testing (BE) | Rust built-in tests                 |
| Testing (FE) | Vitest, React Testing Library, Playwright |
| Infra        | Docker, Docker Compose              |
| CI/CD        | GitHub Actions                      |

---

## Project Structure

```
edukit-platform/
│
├── apps/
│   ├── web/              # Next.js frontend application
│   └── api/              # Rust Axum backend API
│       ├── crates/
│       │   ├── api/          # HTTP layer — routes & handlers
│       │   ├── application/  # Business logic & use cases
│       │   ├── domain/       # Core entities & domain rules
│       │   └── infrastructure/ # Database, external services
│       └── migrations/   # SQLx database migrations
│
├── docs/                 # Project documentation
│   ├── architecture.md
│   ├── api.md
│   ├── contributing.md
│   └── roadmap.md
│
├── .github/              # GitHub workflows & issue templates
│   ├── ISSUE_TEMPLATE/
│   └── workflows/
│
├── docker-compose.yml
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
└── LICENSE
```

---

## Getting Started

### Prerequisites

- [Node.js 20+](https://nodejs.org/)
- [Rust (stable)](https://rustup.rs/)
- [Docker & Docker Compose](https://docs.docker.com/get-docker/)
- [Git](https://git-scm.com/)

### Quick Start

**1. Clone the repository**

```bash
git clone https://github.com/edukit-open/edukit-platform.git
cd edukit-platform
```

**2. Start infrastructure (PostgreSQL)**

```bash
docker-compose up -d db
```

**3. Start the Rust backend**

```bash
cd apps/api
cp .env.example .env
cargo run
```

The API will be available at `http://localhost:8080`

**4. Start the Next.js frontend**

```bash
cd apps/web
cp .env.example .env.local
npm install
npm run dev
```

The app will be available at `http://localhost:3000`

---

## Running Tests

**Backend (Rust)**
```bash
cd apps/api
cargo test
```

**Frontend**
```bash
cd apps/web
npm run test        # Unit tests
npm run test:e2e    # End-to-end tests
npm run lint        # Linting
```

---

## API Documentation

The Rust backend exposes a REST API at `/api/v1`.

| Resource        | Endpoint                      |
|-----------------|-------------------------------|
| Health Check    | `GET /health`                 |
| Kits            | `GET /api/v1/kits`            |
| Kit Details     | `GET /api/v1/kits/:id`        |
| Create Kit      | `POST /api/v1/kits`           |
| Activities      | `GET /api/v1/activities`      |
| Auth Register   | `POST /api/v1/auth/register`  |
| Auth Login      | `POST /api/v1/auth/login`     |

Full API documentation is available in [`docs/api.md`](./docs/api.md).

---

## Roadmap

| Phase | Description                              | Status        |
|-------|------------------------------------------|---------------|
| 1     | Foundation — scaffold, design system, kits listing | 🚧 In Progress |
| 2     | Discovery — search, filters, kit details | 📋 Planned    |
| 3     | Creation — kit creator, form validation  | 📋 Planned    |
| 4     | Community — auth, profiles, ratings      | 📋 Planned    |
| 5     | Scale — public API, multi-language, analytics | 📋 Planned |

Full roadmap: [`docs/roadmap.md`](./docs/roadmap.md)

---

## Contributing

We welcome contributors of all experience levels — frontend, backend, design, testing, documentation, and more.

You do not need to know Rust to contribute. There are issues for every skill level.

Before contributing:
1. Read [CONTRIBUTING.md](./CONTRIBUTING.md)
2. Read [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)
3. Browse [open issues](https://github.com/edukit-open/edukit-platform/issues)
4. Find an issue labeled `good first issue` to start

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the full contribution guide.

---

## Code of Conduct

This project follows a [Code of Conduct](./CODE_OF_CONDUCT.md). By participating, you agree to uphold a welcoming and respectful environment for everyone.

---

## License

This project is licensed under the [MIT License](./LICENSE).

---

## Contributors

EduKit exists because of its contributors. Every contribution — code, documentation, design, testing, translation, or ideas — helps make educational resources more accessible.

| Avatar | Name | Role | GitHub |
|--------|------|------|--------|
| <img src="https://github.com/abrak01.png" width="48" height="48" style="border-radius:50%" alt="abrak01"> | **Abrak Fredrick** | Project Lead & Architect | [@abrak01](https://github.com/abrak01) |
| <img src="https://github.com/identicons/fatima-aliyu.png" width="48" height="48" style="border-radius:50%" alt="Fatima Aliyu"> | **Fatima Aliyu** | Frontend Engineer (Next.js / TypeScript) | [@fatima-aliyu](https://github.com/fatima-aliyu) |
| <img src="https://github.com/identicons/chukwuemeka-obi.png" width="48" height="48" style="border-radius:50%" alt="Chukwuemeka Obi"> | **Chukwuemeka Obi** | Backend Engineer (Rust / Axum) | [@chukwuemeka-obi](https://github.com/chukwuemeka-obi) |
| <img src="https://github.com/identicons/ngozi-adeyemi.png" width="48" height="48" style="border-radius:50%" alt="Ngozi Adeyemi"> | **Ngozi Adeyemi** | UI/UX Design & Accessibility | [@ngozi-adeyemi](https://github.com/ngozi-adeyemi) |

Want to see your name here? Browse our [open issues](https://github.com/abrakk/abrakk-backend/issues), pick one labeled `good first issue`, and open a Pull Request.

**Thank you for helping build a better learning ecosystem.**
