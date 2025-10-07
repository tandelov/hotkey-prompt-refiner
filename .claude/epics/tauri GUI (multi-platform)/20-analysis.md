---
issue: 20
task: Tauri project scaffolding and Svelte UI setup
analyzed: 2025-10-05T20:02:15Z
complexity: medium
parallelizable: true
---

# Work Stream Analysis: Issue #20

## Overview
This task establishes the foundational Tauri + Svelte project structure. The work can be broken into sequential phases due to dependencies (Tauri init must happen before frontend setup).

## Work Streams

### Stream 1: Tauri Project Initialization (Sequential - Must Complete First)
**Agent:** general-purpose
**Estimated Time:** 2-3 hours
**Status:** Can start immediately

**Scope:**
- Install Tauri CLI and initialize project with `create-tauri-app`
- Select Vite + Svelte template
- Configure `tauri.conf.json` for app metadata and window settings
- Update `src-tauri/Cargo.toml` with Tauri 2.x dependencies
- Verify Rust backend builds successfully

**Files to Create/Modify:**
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`
- `src-tauri/src/main.rs`
- Root level config files (from template)

**Deliverables:**
- Working Tauri project structure
- Backend builds successfully
- Basic window opens

**Blocking:** Nothing - this is the entry point

---

### Stream 2: Frontend Setup & Routing (Sequential - After Stream 1)
**Agent:** general-purpose
**Estimated Time:** 2-3 hours
**Status:** Blocked by Stream 1

**Scope:**
- Set up Svelte routing (svelte-routing or svelte-spa-router)
- Create `src-ui/` directory structure
- Implement `App.svelte` with router
- Create placeholder views (Settings.svelte, History.svelte)
- Configure `vite.config.js` for optimal bundle size
- Update `package.json` with frontend dependencies

**Files to Create/Modify:**
- `src-ui/App.svelte`
- `src-ui/main.js`
- `src-ui/views/Settings.svelte`
- `src-ui/views/History.svelte`
- `vite.config.js`
- `package.json`

**Deliverables:**
- Routing works between views
- Hot reload functional
- Development server runs

**Blocking:** Stream 1 (needs Tauri scaffold first)

---

## Execution Strategy

**Phase 1:** Execute Stream 1 (Tauri Initialization)
- Single agent focus
- Validate backend builds before proceeding

**Phase 2:** Execute Stream 2 (Frontend Setup)
- After Stream 1 completes
- Single agent to maintain consistency

**Total Estimated Time:** 4-6 hours (sequential execution)

## Coordination Notes

- No parallel execution possible - streams are dependent
- Stream 1 must fully complete before Stream 2 begins
- Use worktree: `../epic-tauri-gui-multi-platform/`
- Commit frequently with format: `Issue #20: {specific change}`

## Risk Mitigation

- **Tauri 2.x compatibility:** Ensure using latest stable Tauri 2.x release
- **Routing library choice:** Use lightweight router (svelte-spa-router recommended)
- **Build configuration:** Follow Tauri best practices for Vite config

## Definition of Done (Consolidated)

- [ ] Tauri 2.x project initialized and builds successfully
- [ ] Development server runs with hot reload
- [ ] Production build generates functional application
- [ ] Routing navigates between Settings and History views
- [ ] Project structure follows `src/` (Rust) and `src-ui/` (Svelte) pattern
- [ ] README updated with setup instructions
- [ ] All changes committed to worktree branch
