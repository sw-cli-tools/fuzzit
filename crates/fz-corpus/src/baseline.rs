use crate::CaseInput;

use crate::basic::{empty_inputs, single_char_inputs, whitespace_inputs};
use crate::encoding::{invalid_utf8_inputs, newline_variants, null_byte_inputs};
use crate::structure::{deep_nesting, huge_input, numeric_boundary_inputs};
use crate::text::{delimiter_inputs, escape_inputs, weird_identifiers};

const DEFAULT_HUGE_SIZE: usize = 65_536;
const DEFAULT_NESTING_DEPTH: usize = 64;

pub fn generate_baseline_corpus() -> Vec<CaseInput> {
    let mut corpus = Vec::new();

    corpus.extend(empty_inputs());
    corpus.extend(whitespace_inputs());
    corpus.extend(single_char_inputs());
    corpus.push(huge_input(DEFAULT_HUGE_SIZE));
    corpus.extend(delimiter_inputs());
    corpus.extend(escape_inputs());
    corpus.extend(invalid_utf8_inputs());
    corpus.extend(null_byte_inputs());
    corpus.extend(newline_variants());
    corpus.extend(numeric_boundary_inputs());
    corpus.extend(deep_nesting(DEFAULT_NESTING_DEPTH));
    corpus.extend(weird_identifiers());

    corpus
}
