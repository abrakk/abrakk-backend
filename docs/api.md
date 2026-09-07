# API Reference

Base URL: `http://localhost:8080`

All responses are JSON. Authenticated endpoints require `Authorization: Bearer <token>`.

---

## Health

### GET /health

Returns the service health status.

**Response 200**
```json
{
  "status": "ok",
  "service": "edukit-api",
  "version": "0.1.0"
}
```

---

## Authentication

### POST /api/v1/auth/register

Register a new user account.

**Request Body**
```json
{
  "email": "user@example.com",
  "username": "myusername",
  "password": "securepassword123",
  "display_name": "My Name"
}
```

**Response 201**
```json
{
  "access_token": "eyJ...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "myusername",
    "display_name": "My Name"
  }
}
```

---

### POST /api/v1/auth/login

Authenticate with email and password.

**Request Body**
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response 200** — same shape as register response.

---

## Kits

### GET /api/v1/kits

List published kits. Supports filtering via query parameters.

**Query Parameters**

| Parameter | Type | Description |
|---|---|---|
| `subject` | string | Filter by subject area |
| `difficulty` | string | `beginner`, `intermediate`, or `advanced` |
| `age_min` | integer | Minimum age |
| `age_max` | integer | Maximum age |
| `language` | string | Language code (e.g. `en`, `yo`) |
| `search` | string | Full-text search |
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Results per page (default: 20, max: 100) |

**Response 200**
```json
[
  {
    "id": "uuid",
    "title": "Learning Numbers with Local Objects",
    "description": "An activity kit for teaching number recognition...",
    "subject": "mathematics",
    "difficulty": "beginner",
    "age_min": 4,
    "age_max": 6,
    "language": "en",
    "learning_objectives": ["Count from 1 to 10", "Match numbers to objects"],
    "materials_required": ["Counters", "Number cards"],
    "author_id": "uuid",
    "is_published": true,
    "download_count": 42,
    "created_at": "2026-09-07T10:00:00Z",
    "updated_at": "2026-09-07T10:00:00Z"
  }
]
```

---

### GET /api/v1/kits/:id

Get a single kit by ID.

**Response 200** — single kit object (same shape as above).

**Response 404**
```json
{ "error": "Kit with id <uuid> not found", "status": 404 }
```

---

### POST /api/v1/kits

Create a new kit. **Requires authentication.**

**Request Body**
```json
{
  "title": "My Learning Kit",
  "description": "A kit for teaching basic literacy skills.",
  "subject": "literacy",
  "difficulty": "beginner",
  "age_min": 5,
  "age_max": 7,
  "language": "en",
  "learning_objectives": ["Recognise letters A-Z", "Blend simple words"],
  "materials_required": ["Alphabet cards", "Whiteboard"]
}
```

**Response 201** — created kit object.

---

### PATCH /api/v1/kits/:id

Update a kit. **Requires authentication. Only the kit's author may update it.**

All fields are optional.

**Response 200** — updated kit object.

**Response 403** — if the requester is not the kit's author.

---

### DELETE /api/v1/kits/:id

Delete a kit. **Requires authentication. Only the kit's author may delete it.**

**Response 204** — no content.

---

### GET /api/v1/kits/:id/download

Increment the download counter and return the kit.

**Response 200** — kit object with updated `download_count`.

---

## Error Responses

All errors follow this shape:

```json
{
  "error": "Human-readable error message",
  "status": 400
}
```

| Status | Meaning |
|---|---|
| 400 | Bad request / validation error |
| 401 | Missing or invalid token |
| 403 | Access denied |
| 404 | Resource not found |
| 409 | Conflict (e.g. email already exists) |
| 500 | Internal server error |
