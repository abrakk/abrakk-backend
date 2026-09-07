# Architecture

## Overview

EduKit uses a decoupled, layered architecture:

```
┌──────────────────────┐
│   Next.js Frontend   │   (apps/web)
│  React + TypeScript  │
│     Tailwind CSS     │
└──────────┬───────────┘
           │
      REST API / JSON
           │
           ▼
┌──────────────────────┐
│    Rust Backend      │   (apps/api)
│   Axum + Tokio       │
│   SQLx + PostgreSQL  │
└──────────┬───────────┘
           │
           ▼
┌──────────────────────┐
│     PostgreSQL       │
└──────────────────────┘
```

The frontend and backend are independently deployable services connected only by the REST API contract.

---

## Rust Backend Architecture

The backend follows a Clean Architecture-inspired layered approach using a Cargo workspace.

```
apps/api/
└── crates/
    ├── api/            HTTP layer — routes, handlers, middleware
    ├── application/    Business logic — services, DTOs
    ├── domain/         Core entities, errors, repository traits
    └── infrastructure/ PostgreSQL repository implementations
```

### Request Flow

```
HTTP Request
     │
     ▼
routes/*.rs        — URL routing
     │
     ▼
handlers/*.rs      — Extract params, call service, return response
     │
     ▼
application/       — Business logic, validation, orchestration
     │
     ▼
domain/repositories — Trait definitions (interfaces)
     │
     ▼
infrastructure/    — SQLx + PostgreSQL implementations
     │
     ▼
PostgreSQL
```

### Dependency Rule

Dependencies only point inward:

- `api` depends on `application`
- `application` depends on `domain`
- `infrastructure` depends on `domain`
- `domain` has no internal dependencies

This means the domain layer is completely isolated from HTTP and database concerns.

### Crate Responsibilities

| Crate | Responsibility |
|---|---|
| `domain` | Core entities (`Kit`, `User`, `Activity`), domain errors, repository traits |
| `application` | Services (`KitService`, `AuthService`), DTOs, business rules |
| `infrastructure` | `PostgresKitRepository`, `PostgresUserRepository` |
| `api` | Axum routes, handlers, middleware, `AppState`, `main.rs` |

---

## Next.js Frontend Architecture

The frontend uses a feature-based modular structure.

```
apps/web/src/
├── app/              Next.js App Router pages
├── components/       Shared UI components (Button, Badge, Input, Navbar, Footer)
└── features/         Feature domains
    ├── kits/         Kit browsing, details, creation
    │   ├── components/
    │   ├── services/   API calls to the Rust backend
    │   └── types.ts
    ├── activities/   (planned)
    └── users/        (planned)
```

### Data Fetching

- **Server components** (default): fetch directly from the Rust API using `fetch()` with ISR caching.
- **Client components** (`"use client"`): used for interactive elements like filters and forms.

---

## Database Schema

```
users
├── id (uuid, pk)
├── email (unique)
├── username (unique)
├── password_hash
├── display_name
├── bio
├── role (contributor | educator | admin)
├── is_active
├── created_at
└── updated_at

kits
├── id (uuid, pk)
├── title
├── description
├── subject (enum)
├── difficulty (enum)
├── age_min / age_max
├── language
├── learning_objectives (text[])
├── materials_required (text[])
├── author_id (fk → users.id)
├── is_published
├── download_count
├── created_at
└── updated_at

activities
├── id (uuid, pk)
├── kit_id (fk → kits.id)
├── title
├── description
├── instructions
├── duration_minutes
├── order_index
├── created_at
└── updated_at
```

---

## Authentication

- Passwords are hashed with **Argon2** (winner of the Password Hashing Competition).
- Sessions are stateless **JWT** tokens passed as `Authorization: Bearer <token>`.
- Tokens expire after 24 hours (configurable via `JWT_EXPIRATION_HOURS`).
- The `AuthenticatedUser` extractor in Axum validates tokens on protected routes.

---

## Configuration

All configuration is provided via environment variables. See `.env.example` in `apps/api/` and `apps/web/`.

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | postgres://... | PostgreSQL connection string |
| `HOST` | 127.0.0.1 | API bind host |
| `PORT` | 8080 | API bind port |
| `JWT_SECRET` | — | Must be set in production |
| `JWT_EXPIRATION_HOURS` | 24 | Token TTL |
| `RUST_LOG` | info | Log level |
| `NEXT_PUBLIC_API_URL` | http://localhost:8080 | Frontend → API URL |
