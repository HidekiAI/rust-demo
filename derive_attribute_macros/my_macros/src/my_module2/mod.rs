// src/my_module2/mod.rs

// Re-export the procedural macro
//pub use sub_module2::my_macro; // Assuming your procedural macro is named "my_macro"
pub mod sub_module2;
pub use sub_module2::my_macro;

// Other module-level declarations...
