use crate::CaseInput;

use crate::deep_nesting::deep_nesting;
use crate::delimiter::delimiter_inputs;
use crate::empty::empty_inputs;
use crate::escape::escape_inputs;
use crate::huge::huge_input;
use crate::invalid_utf8::invalid_utf8_inputs;
use crate::newline::newline_variants;
use crate::null_byte::null_byte_inputs;
use crate::numeric_boundary::numeric_boundary_inputs;
use crate::single_char::single_char_inputs;
use crate::weird_id::weird_identifiers;
use crate::whitespace::whitespace_inputs;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_is_non_empty() {
        let corpus = generate_baseline_corpus();
        assert!(!corpus.is_empty());
    }

    #[test]
    fn corpus_is_substantial() {
        let corpus = generate_baseline_corpus();
        assert!(corpus.len() > 100);
    }

    #[test]
    fn all_descriptions_non_empty() {
        let corpus = generate_baseline_corpus();
        for case in &corpus {
            assert!(
                !case.description.is_empty(),
                "empty description for data: {:?}",
                case.data
            );
        }
    }

    #[test]
    fn contains_empty_input() {
        let corpus = generate_baseline_corpus();
        assert!(corpus.iter().any(|c| c.data.is_empty()));
    }

    #[test]
    fn contains_huge_input() {
        let corpus = generate_baseline_corpus();
        assert!(corpus.iter().any(|c| c.data.len() >= DEFAULT_HUGE_SIZE));
    }

    #[test]
    fn no_generator_panics() {
        let corpus = generate_baseline_corpus();
        assert!(!corpus.is_empty());
    }

    #[test]
    fn descriptions_are_unique() {
        let corpus = generate_baseline_corpus();
        let mut seen = std::collections::HashSet::new();
        for case in &corpus {
            assert!(
                seen.insert(&case.description),
                "duplicate description: {}",
                case.description
            );
        }
    }
}
