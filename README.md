# KriptOx

KriptOx — Information Security project (university)

KriptOx is a desktop application built as a university information-security project. It provides secure file encryption/decryption, local file management, key management, and network transfer functionality. The UI is a Svelte + Vite frontend bundled into a Tauri desktop app; the backend cryptography and system integrations are implemented in Rust.

**Key features**

- **Encrypt / Decrypt files:** Symmetric block and stream cipher implementations for file encryption and decryption.
- **Key Manager:** Create, store, and manage keys using the app's key manager subsystem.
- **Network Transfer (TCP):** Send and receive encrypted files over the network via the built-in sender/receiver modules.
- **Local File System Explorer:** Browse and operate on local files from the UI.

Project layout (selected files & folders)

- `src/` (frontend)
  - `src/lib/pages/KeyManager.svelte` — Key manager UI page
  - `src/lib/pages/LocalFileSystem.svelte` — Local file explorer UI
  - `src/lib/pages/NetworkTransfer.svelte` — Network send/receive UI
  - `src/lib/components/ui/` — Reusable UI components (navbar, file-explorer, dialogs)
- `src-tauri/` (Rust backend + Tauri configuration)
  - `src-tauri/src/crypto/` — Cipher implementations (e.g., `aes256.rs`, `xtea.rs`, stream `a5_1.rs`), padding (`pkcs7.rs`), hashing (`blake_256.rs`), factories and APIs
  - `src-tauri/src/network/` — Network sender/receiver implementations
  - `src-tauri/src/key_manager/` — Key manager logic and key types
  - `src-tauri/src/files/` — File explorer, file commands and watching
  - `src-tauri/tauri.conf.json` — Tauri configuration

Architecture overview

- Frontend: Svelte + Vite app in `src/` provides the UI (pages for key management, file browsing, network transfer). The app communicates with the Rust backend via Tauri commands.
- Backend: Rust code under `src-tauri/src/` implements cryptographic primitives, file and key management, and network transfer. This keeps sensitive operations in a native, auditable layer.
- Build: Vite builds the web frontend, then Tauri packages the app into a native desktop binary with the Rust backend.

Supported cryptography (implemented in Rust modules)

- Block ciphers: AES-256, XTEA (examples present in `src-tauri/src/crypto/block/`)
- Stream ciphers: A5/1 (in `src-tauri/src/crypto/stream/`)
- Hashing: BLAKE variants (e.g., `blake_256.rs`)
- Padding: PKCS#7

How to run (development)
Prerequisites

- Node.js (recommended v16+), npm or yarn
- Rust toolchain (stable) and Cargo
- Tauri prerequisites for your OS (see Tauri docs)

Start the app for development (typical commands)

```bash
# From repository root
npm install
# Run the frontend + Tauri dev server (this runs Vite and the Rust backend)
npm run tauri dev
```

If your project uses separate scripts for the web dev server, an alternative two-step flow is:

```bash
npm install
npm run dev        # starts Vite frontend
cd src-tauri
cargo run          # runs the Rust backend for integration testing
```

How to build (production)

```bash
npm install
npm run build          # builds the web assets with Vite
# then package native app with Tauri
npm run tauri build
```

After a successful Tauri build, native bundles or binaries are placed under `src-tauri/target/release/bundle` (or the platform-specific output shown by Tauri).

Security notes

- Keep private keys secure — the key manager persists keys on disk; treat them like sensitive material.
- This project is an academic implementation: for production or real-world security use, perform an independent security audit and ensure best-practice key storage and secure transport (e.g., authenticated channels).

Contributing & next steps

- Run the app locally (see commands above) and exercise encryption, key management and network transfer flows.
- If you want, I can add: example key import/export, automated tests for crypto modules, or CI build scripts.

License

- See `LICENSE` in the repository root for license details.

Contact

- For questions about this repository or the assignment, contact the project author or maintainers listed in repository metadata.
  For questions about this repository or the assignment, contact the project author or maintainers listed in repository metadata.

Template

- Project scaffolded from a Tauri + Svelte starter template. The UI uses the `bits-ui` component library alongside local components under `src/lib/components/ui/`.

Requirements

- Node.js (recommended v16+), npm or yarn
- Rust toolchain (stable) and Cargo
- Tauri prerequisites for your OS (see Tauri docs)

Setup

1. Clone the repository and install dependencies:

```bash
git clone <repo-url>
cd <repo>
npm install
```

Useful commands

- Start development (Vite + Tauri):

```bash
npm run tauri dev
```

- Build production assets and package with Tauri:

```bash
npm run build
npm run tauri build
```

Other links

- Svelte 5: https://svelte.dev/docs
- Tauri: https://tauri.app/start/

License

- See `LICENSE` in the repository root for license details.
