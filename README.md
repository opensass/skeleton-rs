<div align="center">

# 🦴 Skeleton RS

[![Crates.io](https://img.shields.io/crates/v/skeleton-rs)](https://crates.io/crates/skeleton-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/skeleton-rs)](https://crates.io/crates/skeleton-rs)
![Crates.io License](https://img.shields.io/crates/l/skeleton-rs)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)

[![Join our Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->
![logo](https://raw.githubusercontent.com/opensass/skeleton-rs/refs/heads/main/assets/logo.webp)

</div>

## 🎬 Demo

| Framework | Live Demo |
| --- | --- |
| Yew | [![Netlify Status](https://api.netlify.com/api/v1/badges/a0efc7e9-f20e-4dd9-93e1-c8f4fde7506f/deploy-status)](https://skeleton-rs.netlify.app) |
| Dioxus | [![Netlify Status](https://api.netlify.com/api/v1/badges/a0efc7e9-f20e-4dd9-93e1-c8f4fde7506f/deploy-status)](https://skeleton-dio.netlify.app) |
| Leptos | TODO |

## 📜 Intro

**Skeleton RS** is a **lightweight**, **highly customizable** loading placeholder component for WASM-based UI frameworks like **Yew**, **Dioxus**, and **Leptos**. It helps deliver a smooth loading experience with animated placeholders, responsive sizing, and fine-grained visibility control using `IntersectionObserver`.

## 🤔 Why Use Skeleton RS?

The following features make Skeleton RS an essential building block for modern frontend apps built with Rust and WebAssembly:

1. **✨ Smooth UX**: Mimics content structure while loading, avoiding layout shifts.
1. **👁️ Animate on Visibility**: Supports visibility-based animations using `IntersectionObserver`.
1. **🎛️ Fully Customizable**: Control height, width, border radius, animation style, and more.
1. **📱 Responsive Ready**: Automatically adapts to screen sizes when enabled.
1. **⏳ Delay-Aware Rendering**: Prevents unnecessary skeleton flashes using `delay_ms`.

## Yew Usage

<!-- absolute url for docs.rs cause YEW.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/skeleton-rs/blob/main/YEW.md) to integrate this component into your Yew app.

## 🧬 Dioxus Usage

<!-- absolute url for docs.rs cause DIOXUS.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/skeleton-rs/blob/main/DIOXUS.md) to integrate this component into your Dioxus app.

## 🌱 Leptos Usage (TODO)

<!-- absolute url for docs.rs cause LEPTOS.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/skeleton-rs/blob/main/LEPTOS.md) to integrate this component into your Leptos app.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make Skeleton RS better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

Skeleton RS is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
