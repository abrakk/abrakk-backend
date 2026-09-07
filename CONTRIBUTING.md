# Contributing to EduKit

Thank you for your interest in contributing to EduKit. We welcome contributions of all kinds — code, documentation, design, testing, translations, and ideas.

This guide explains how to contribute effectively.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [What We Are Looking For](#what-we-are-looking-for)
- [How to Find an Issue](#how-to-find-an-issue)
- [Setting Up the Project](#setting-up-the-project)
- [Contribution Workflow](#contribution-workflow)
- [Branch Naming](#branch-naming)
- [Commit Messages](#commit-messages)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Code Standards](#code-standards)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Getting Help](#getting-help)

---

## Code of Conduct

Please read our [Code of Conduct](./CODE_OF_CONDUCT.md) before contributing. All contributors are expected to uphold it.

---

## What We Are Looking For

We welcome contributions including:

- Bug fixes
- New features from the roadmap
- Accessibility improvements
- Mobile responsiveness improvements
- Performance improvements
- Test coverage
- Documentation improvements
- UI/UX improvements
- Translations and localisation
- DevOps and infrastructure improvements

If you are unsure whether your idea is suitable, open a Discussion or comment on a related issue before starting work.

---

## How to Find an Issue

Browse the [issues list](https://github.com/edukit-open/edukit-platform/issues).

Useful labels:

| Label              | Description                                      |
|--------------------|--------------------------------------------------|
| `good first issue` | Suitable for contributors new to the project     |
| `help wanted`      | We would appreciate help here                    |
| `bug`              | Something is broken                              |
| `feature`          | New feature request                              |
| `frontend`         | Relates to the Next.js frontend                  |
| `backend`          | Relates to the Rust API                          |
| `documentation`    | Improvements to docs                             |
| `accessibility`    | Accessibility improvements                       |
| `testing`          | Test coverage                                    |
| `design`           | UI/UX design work                                |

**Before starting work on an issue:**

1. Check the issue is not already assigned or being worked on.
2. Leave a comment saying you would like to work on it.
3. Wait for confirmation from a maintainer.

This avoids duplicate work.

---

## Setting Up the Project

### Prerequisites

- Node.js 20 or newer
- Rust (stable) via [rustup](https://rustup.rs/)
- Docker and Docker Compose
- Git

### Clone and Setup

```bash
# Clone the repository
git clone https://github.com/edukit-open/edukit-platform.git
cd edukit-platform

# Start the database
docker-compose up -d db

# Set up the backend
cd apps/api
cp .env.example .env
cargo build

# Set up the frontend
cd ../web
cp .env.example .env.local
npm install
```

### Running the Project

**Backend:**
```bash
cd apps/api
cargo run
```

**Frontend:**
```bash
cd apps/web
npm run dev
```

---

## Contribution Workflow

```
1. Find an issue and comment to claim it
        ↓
2. Fork the repository
        ↓
3. Create a branch from main
        ↓
4. Implement your changes
        ↓
5. Write or update tests
        ↓
6. Run tests and linting
        ↓
7. Commit your changes
        ↓
8. Push your branch
        ↓
9. Open a Pull Request
        ↓
10. Address review feedback
        ↓
11. Merged!
```

---

## Branch Naming

Use descriptive branch names with a consistent prefix:

| Type          | Format                         | Example                         |
|---------------|--------------------------------|---------------------------------|
| Feature       | `feature/<short-description>`  | `feature/kit-search`            |
| Bug Fix       | `fix/<short-description>`      | `fix/mobile-navigation`         |
| Documentation | `docs/<short-description>`     | `docs/improve-setup-guide`      |
| Testing       | `test/<short-description>`     | `test/kit-service-unit-tests`   |
| Chore         | `chore/<short-description>`    | `chore/update-dependencies`     |
| Refactor      | `refactor/<short-description>` | `refactor/kit-repository`       |

---

## Commit Messages

Use clear, descriptive commit messages.

**Format:**
```
<type>: <short description>

<optional longer description>
```

**Types:**
- `feat:` — A new feature
- `fix:` — A bug fix
- `docs:` — Documentation only changes
- `style:` — Formatting, no logic change
- `refactor:` — Code change, not a fix or feature
- `test:` — Adding or updating tests
- `chore:` — Build, tooling, dependencies

**Examples:**
```
feat: add subject filtering to kit search
fix: correct mobile navigation overlap
docs: improve API setup guide
test: add unit tests for kit service
```

---

## Pull Request Guidelines

When opening a Pull Request:

1. Fill in the PR template completely.
2. Link the issue your PR addresses (e.g. `Closes #42`).
3. Keep PRs focused. One PR should address one issue.
4. Add screenshots for UI changes.
5. Make sure all tests pass before requesting review.
6. Keep the diff clean — avoid unrelated changes.

PRs will be reviewed by a maintainer. Feedback may be requested before merging.

---

## Code Standards

### Frontend (TypeScript / Next.js)

- Use TypeScript strictly. Avoid `any`.
- Follow the existing component structure under `features/`.
- Use Tailwind CSS for styling — avoid inline styles.
- Components should be accessible (ARIA labels, keyboard navigation, etc.).
- Run `npm run lint` before committing.

### Backend (Rust)

- Run `cargo fmt` before committing.
- Run `cargo clippy` and address all warnings.
- Follow the layered architecture: routes → handlers → services → repositories.
- Write unit tests for services and integration tests for API routes.
- Use `tracing` for logging — avoid `println!` in production code.
- Document public functions and types with doc comments (`///`).

---

## Reporting Bugs

Before reporting:

- Search existing issues to avoid duplicates.
- Make sure the bug is reproducible.

When reporting, include:

- A clear title describing the problem
- Steps to reproduce
- Expected behavior
- Actual behavior
- Browser and OS (for frontend issues)
- Error messages or screenshots

Use the [Bug Report template](.github/ISSUE_TEMPLATE/bug.yml).

---

## Suggesting Features

Feature requests are welcome.

When suggesting, explain:

- The problem you are trying to solve
- Your proposed solution
- Who benefits from this feature
- Any alternatives you considered

Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature.yml).

Large features should be discussed in an issue before implementation begins.

---

## Getting Help

If you have questions:

- Comment on the relevant issue
- Open a Discussion in the repository
- Refer to the documentation in the [`docs/`](./docs/) folder

We are happy to help contributors get started.

---

Thank you for contributing to EduKit. Every contribution helps make educational resources more accessible.
