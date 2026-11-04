//! Shared NVForever runtime. The implementation remains in main.rs for now so the
//! desktop and Android entry points execute exactly the same game initialization path.
//! As Android-specific services are added, they belong in dedicated platform modules.
include!("main.rs");
