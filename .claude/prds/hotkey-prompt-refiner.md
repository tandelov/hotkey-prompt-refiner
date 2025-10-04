---
name: hotkey-prompt-refiner
description: Lightweight cross-platform desktop application for AI-powered text processing via global hotkeys
status: ready-for-development
created: 2025-10-04T15:46:05Z
updated: 2025-10-04T15:46:05Z
version: 1.0
---

# **Product Requirements Document: Hotkey Prompt Refiner**

**Version:** 1.0
**Status:** Ready for Development

## 1. Introduction / Overview
The Hotkey Prompt Refiner is a lightweight, cross-platform desktop application designed to streamline workflows by integrating AI text processing directly into any application. Users can copy text to their clipboard, press a configurable global hotkey, and have the clipboard content sent with a predetermined prompt to the Anthropic Claude API. The AI-generated response is then automatically pasted at the current cursor location, enabling rapid, in-context text transformation, summarization, translation, and more. The application is designed as a headless, background service with a minimal resource footprint.

## 2. Goals and Objectives
*   **Primary Goal:** To significantly reduce the friction of using AI for text manipulation by allowing users to invoke AI assistance on clipboard text from any application via a simple hotkey.
*   **Key Objectives:**
    *   **Speed:** Provide a near-instantaneous way to capture, process, and paste text without context switching.
    *   **Efficiency:** Run as a lightweight background service with a memory footprint under 30 MB and a binary size under 5 MB.
    *   **Simplicity:** The core interaction should be a single hotkey press after copying text.
    *   **Platform Support:** Prioritize flawless operation on macOS, with functional support for Windows and Linux.

## 3. Target Audience / User Personas
The primary target audience consists of **power users, developers, writers, and researchers** who work extensively with text and are looking to optimize their workflows. They are comfortable with concepts like API keys and simple configuration.

## 4. User Stories / Use Cases
*   **As an LLM chatbot user,** I want to improve my prompt to find smallest possible set of high-signal tokens that maximize the likelihood of some desired outcome, with prompt pre-processing with Anthropic LLMs, and refining prompt manually by creating new chat just for refinement, re-crafting refinement meta-prompt and copypasting updated prompt to is problem that can be improved with binding auto-refinement at some hot key.


## 5. Functional Requirements

### 5.1. Core Functionality
*   **FR-1: Global Hotkey Registration:** The application must register a system-wide hotkey combination.
*   **FR-2: Clipboard Text Capture:** Upon hotkey activation, the application must read the text content currently in the system clipboard.
*   **FR-3: Prompt Formatting:** The captured clipboard text must be packaged with a predetermined prompt template before being sent to the API. Prompt template is in distinct config for easier modifications.
*   **FR-4: API Integration:** The application must send the formatted prompt to the Anthropic Claude API for processing. Use allm library.
*   **FR-5: In-Place Pasting:** The application must paste the response from the API directly at the user's current cursor location.
*   **FR-6: Background Operation:** The application must run as a headless background service.

### 5.2. Configuration
*   **FR-7: API Key Management:** The application must be configured with an Anthropic API key via an environment variable (`ANTHROPIC_API_KEY`) or a `.env` file.

### 5.3. Platform-Specific Requirements
*   **FR-8 (macOS):** The application must correctly handle macOS's requirement to run the hotkey event loop on the main thread.
*   **FR-9 (macOS):** The application must detect if it has the necessary Accessibility permissions to function.

## 6. Non-Functional Requirements
*   **NFR-1 (Performance):**
    *   **Memory Usage:** The application's idle memory usage shall be under 30 MB RAM.
    *   **Binary Size:** The compiled release binary shall be under 5 MB.
    *   **CPU Usage:** Idle CPU usage shall be less than 1%.
    *   **Latency:** The time from hotkey press to the start of the API call should be less than 200ms.
*   **NFR-2 (Platform Support):** The application must be compilable and runnable on macOS (primary), Windows (secondary), and Linux (secondary).
*   **NFR-3 (Security):**
    *   API keys must not be stored in plaintext in the source code.
    *   All communication with the Anthropic API must be over HTTPS.
    *   The application will not store any user-selected text or API responses locally.
*   **NFR-4 (Usability):** The application must provide clear startup and error messages in the terminal when it is launched (e.g., confirming hotkey registration, reporting a missing API key).

## 7. Success Metrics
*   The application registers a global hotkey and successfully captures text from the clipboard.
*   The application communicates with the Anthropic API and correctly pastes the response at the cursor.
*   Memory usage remains below 30 MB in typical idle operation.
*   Release binary size is below 5 MB.
*   The tool is functional across macOS, Windows, and Linux.

## 8. Future Considerations (Roadmap)
*   **Configuration File:** Implement a `config.toml` file to allow users to easily change the hotkey, prompt templates, API model, and other behaviors.
*   **Enhanced UX:**
    *   Develop a system tray or menu bar icon for application status and quick settings.
    *   Use native OS notifications to display API responses or error messages.
*   **Multiple Profiles:** Allow users to configure multiple hotkeys, each tied to a different prompt template (e.g., Cmd+Shift+S for "Summarize", Cmd+Shift+T for "Translate").
