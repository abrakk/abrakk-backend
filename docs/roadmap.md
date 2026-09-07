# Roadmap

This roadmap outlines the planned development phases for EduKit.

Each phase is designed to produce a set of well-scoped, independently completable tasks suitable for open-source contributors.

---

## Phase 1 — Foundation 🚧 In Progress

Goal: a working platform with browseable, downloadable kits.

- [x] Project scaffold and monorepo setup
- [x] Rust API — Axum, SQLx, PostgreSQL
- [x] Database migrations (users, kits, activities)
- [x] JWT authentication (register, login)
- [x] Kits API (CRUD + download counter)
- [x] Next.js frontend scaffold
- [x] Explore page with filters
- [x] Kit detail page
- [x] Shared UI component library (Button, Badge, Input)
- [x] GitHub Actions CI
- [ ] Kit creation form (frontend)
- [ ] User registration and login pages (frontend)
- [ ] Connect frontend auth to Rust API
- [ ] Activities API (backend)
- [ ] Activities display on kit detail page
- [ ] Responsive mobile navigation

---

## Phase 2 — Discovery 📋 Planned

Goal: powerful search and discovery experience.

- [ ] Full-text search (PostgreSQL GIN index)
- [ ] Advanced subject filtering
- [ ] Age range slider component
- [ ] Multi-language filter
- [ ] Difficulty filter
- [ ] Sort options (newest, most downloaded, relevance)
- [ ] Empty state illustrations
- [ ] Pagination component

---

## Phase 3 — Creation 📋 Planned

Goal: allow educators to create and publish kits on the platform.

- [ ] Kit creation multi-step form
- [ ] Rich text description editor
- [ ] Activity builder within kit creator
- [ ] Form validation (client and server)
- [ ] Draft saving
- [ ] Kit preview before publishing
- [ ] Image/cover upload for kits
- [ ] Publish / unpublish toggle

---

## Phase 4 — Community 📋 Planned

Goal: build community features and contributor profiles.

- [ ] User profile pages
- [ ] Contributor dashboard
- [ ] Kit ratings and reviews
- [ ] Kit bookmarks / favourites
- [ ] Email notifications
- [ ] Activity feed
- [ ] Admin dashboard

---

## Phase 5 — Scale 📋 Planned

Goal: scale the platform and improve reach.

- [ ] Public REST API (rate-limited)
- [ ] API documentation (OpenAPI/Swagger UI)
- [ ] Multi-language UI (i18n)
- [ ] Advanced analytics
- [ ] Performance optimisation
- [ ] CDN for downloadable resources
- [ ] Mobile app (React Native — future consideration)

---

## Contributing to the Roadmap

If you have ideas for the roadmap, open a [Feature Request](https://github.com/edukit-open/edukit-platform/issues/new?template=feature.yml) on GitHub.

Items marked as `help wanted` or `good first issue` are particularly suitable for new contributors.
