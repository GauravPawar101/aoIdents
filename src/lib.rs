//! Identity, trust and cryptography for the aoheyra mesh.
//!
//! `ao_idents` owns who a node *is*, how contacts are trusted, and how
//! messages are signed, sealed and replay-checked: node keys, `NodeId`,
//! identity cards, contextual signatures, sealed messages, rotating tags,
//! contact-book logic, the replay guard and the optional hop handshake.
//!
//! # Boundaries
//!
//! - **Pure Rust: no networking, no I/O, no FFI.** `now` and randomness are
//!   always parameters; persistence lives behind the `KeyStore` and
//!   `ContactStore` traits, implemented by the `aoheyra` app.
//! - This crate never depends on `ao_network`, `ao_mesh` or `aoheyra`.
//! - It signs and seals opaque bytes; it never sees packet formats, routing
//!   or app payload schemas.
//!
//! # Security posture
//!
//! The rules in `AGENTS.md` section 4 are hard requirements: suite per the
//! crypto-suite ADR (IDN-01), a suite-ID byte in every structure, domain
//! separation on every signature and KDF use, one canonical encoding,
//! injected randomness, zeroized redacted secrets, constant-time comparison,
//! reject-don't-repair, and typed errors everywhere. Nothing in this crate
//! may panic on external input.
//!
//! ```text
//!                 aoheyra  (app: Rust core + Kotlin UI)
//!                    │  depends on all three
//!         ┌──────────┼───────────┐
//!         ▼          ▼           ▼
//!     aoMesh ───▶ aoNetwork    aoIdents
//!         │                       ▲
//!         └───────────────────────┘
//! ```

// AGENTS.md section 5: unsafe is forbidden in this crate unless there is no
// alternative; forbid at the crate level so a future `unsafe` is a compile
// error, not a review comment.
#![forbid(unsafe_code)]
#![deny(missing_docs)]
