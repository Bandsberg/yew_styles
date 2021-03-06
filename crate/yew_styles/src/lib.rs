//! # Yew Styles
//!
//! Yew Styles is a style framework for yew
//!
//! ## Motivation
//!
//! The purpose of developing this project is first,
//! provide a style framework for yew because there isn't not many options currently,
//! also to create a layout system which is not far of the flexbox concept, and,
//! to take the rust benefits and implement a properties selected by enumeration
//! in the most of the cases which makes fast for developing applications and avoids the practice try and error
//!
//! ## How it works
//!
//! Each component is split in two parts, the logical yew component and its sass module,
//! however, it is not necessary to worry about the sass module only it needs to be include in the project
//!
//! ### How install it
//!
//! 1. Install the sass module: `npm install yew-styles`
//! 2. Add the yew_style crate with the features needed for your project in Cargo.toml file:
//! ```toml
//! yew_styles = { version="0.5.1", features=["button", "navbar", "layouts"]}
//! ```
//! 3. Import the main.css file in you main javascript/typescript file project:
//! ```typescript
//!    import 'node_modules/yew-styles/main.css';
//! ```
//! 4. Ready to import and use in your project 🚀
//!
//! ## Development phase
//!
//! Yew style is in early phase, currently doesn't have enough components to cover all the requirements that could need a website/web application.
//! All contributions are appreciated.
#![recursion_limit = "512"]
mod assets;
mod components;
pub mod styles;
mod utils;

#[cfg(feature = "assets")]
pub use assets::Assets;
#[cfg(feature = "button")]
pub use components::button;
#[cfg(all(feature = "card", feature = "layouts"))]
pub use components::card;
#[cfg(feature = "forms")]
pub use components::forms;
#[cfg(feature = "layouts")]
pub use components::layouts;
#[cfg(all(feature = "navbar", feature = "layouts"))]
pub use components::navbar;
