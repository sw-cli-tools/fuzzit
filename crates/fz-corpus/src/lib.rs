pub mod baseline;
pub mod case;
pub mod deep_nesting;
pub mod delimiter;
pub mod empty;
pub mod escape;
pub mod huge;
pub mod invalid_utf8;
pub mod newline;
pub mod null_byte;
pub mod numeric_boundary;
pub mod single_char;
pub mod weird_id;
pub mod whitespace;

pub use baseline::generate_baseline_corpus;
pub use case::CaseInput;
