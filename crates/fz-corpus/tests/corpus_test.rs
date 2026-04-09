use fz_corpus::generate_baseline_corpus;

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
    assert!(corpus.iter().any(|c| c.data.len() >= 65536));
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

#[test]
fn empty_non_empty() {
    let inputs = fz_corpus::basic::empty_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn empty_contains_empty_vec() {
    let inputs = fz_corpus::basic::empty_inputs();
    assert!(inputs.iter().any(|i| i.data.is_empty()));
}

#[test]
fn empty_descriptions_non_empty() {
    let inputs = fz_corpus::basic::empty_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn whitespace_non_empty() {
    let inputs = fz_corpus::basic::whitespace_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn whitespace_contains_space() {
    let inputs = fz_corpus::basic::whitespace_inputs();
    assert!(inputs.iter().any(|i| i.data == b" "));
}

#[test]
fn whitespace_contains_tab() {
    let inputs = fz_corpus::basic::whitespace_inputs();
    assert!(inputs.iter().any(|i| i.data == b"\t"));
}

#[test]
fn whitespace_descriptions_non_empty() {
    let inputs = fz_corpus::basic::whitespace_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn single_char_non_empty() {
    let inputs = fz_corpus::basic::single_char_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn single_char_covers_printable_ascii() {
    let inputs = fz_corpus::basic::single_char_inputs();
    assert!(inputs.iter().any(|i| i.data == b"a"));
    assert!(inputs.iter().any(|i| i.data == b"Z"));
}

#[test]
fn single_char_each_is_single_byte() {
    let inputs = fz_corpus::basic::single_char_inputs();
    for input in &inputs {
        assert!(input.data.len() <= 1);
    }
}

#[test]
fn single_char_descriptions_non_empty() {
    let inputs = fz_corpus::basic::single_char_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn huge_all_same_byte() {
    let input = fz_corpus::structure::huge_input(100);
    assert!(input.data.iter().all(|&b| b == input.data[0]));
}

#[test]
fn huge_correct_size() {
    let input = fz_corpus::structure::huge_input(500);
    assert_eq!(input.data.len(), 500);
}

#[test]
fn huge_description_contains_size() {
    let input = fz_corpus::structure::huge_input(100);
    assert!(input.description.contains("100"));
}

#[test]
fn huge_handles_zero_size() {
    let input = fz_corpus::structure::huge_input(0);
    assert!(input.data.is_empty());
}

#[test]
fn delimiter_non_empty() {
    let inputs = fz_corpus::text::delimiter_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn delimiter_contains_parens() {
    let inputs = fz_corpus::text::delimiter_inputs();
    assert!(inputs.iter().any(|i| i.data.starts_with(b"(")));
}

#[test]
fn delimiter_contains_braces() {
    let inputs = fz_corpus::text::delimiter_inputs();
    assert!(inputs.iter().any(|i| i.data.starts_with(b"{")));
}

#[test]
fn delimiter_descriptions_non_empty() {
    let inputs = fz_corpus::text::delimiter_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn escape_non_empty() {
    let inputs = fz_corpus::text::escape_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn escape_contains_single_backslash() {
    let inputs = fz_corpus::text::escape_inputs();
    assert!(inputs.iter().any(|i| i.data == b"\\"));
}

#[test]
fn escape_descriptions_non_empty() {
    let inputs = fz_corpus::text::escape_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn invalid_utf8_non_empty() {
    let inputs = fz_corpus::encoding::invalid_utf8_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn invalid_utf8_all_invalid_utf8() {
    let inputs = fz_corpus::encoding::invalid_utf8_inputs();
    for input in &inputs {
        assert!(String::from_utf8(input.data.clone()).is_err());
    }
}

#[test]
fn invalid_utf8_descriptions_non_empty() {
    let inputs = fz_corpus::encoding::invalid_utf8_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn newline_non_empty() {
    let inputs = fz_corpus::encoding::newline_variants();
    assert!(!inputs.is_empty());
}

#[test]
fn newline_contains_lf() {
    let inputs = fz_corpus::encoding::newline_variants();
    assert!(inputs.iter().any(|i| i.data == b"\n"));
}

#[test]
fn newline_contains_crlf() {
    let inputs = fz_corpus::encoding::newline_variants();
    assert!(inputs.iter().any(|i| i.data == b"\r\n"));
}

#[test]
fn newline_descriptions_non_empty() {
    let inputs = fz_corpus::encoding::newline_variants();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn null_byte_non_empty() {
    let inputs = fz_corpus::encoding::null_byte_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn null_byte_contains_single_null() {
    let inputs = fz_corpus::encoding::null_byte_inputs();
    assert!(inputs.iter().any(|i| i.data == b"\0"));
}

#[test]
fn null_byte_all_contain_null() {
    let inputs = fz_corpus::encoding::null_byte_inputs();
    for input in &inputs {
        assert!(input.data.contains(&0));
    }
}

#[test]
fn null_byte_descriptions_non_empty() {
    let inputs = fz_corpus::encoding::null_byte_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn numeric_boundary_non_empty() {
    let inputs = fz_corpus::structure::numeric_boundary_inputs();
    assert!(!inputs.is_empty());
}

#[test]
fn numeric_boundary_contains_zero() {
    let inputs = fz_corpus::structure::numeric_boundary_inputs();
    assert!(inputs.iter().any(|i| i.data == b"0"));
}

#[test]
fn numeric_boundary_contains_negative_one() {
    let inputs = fz_corpus::structure::numeric_boundary_inputs();
    assert!(inputs.iter().any(|i| i.data == b"-1"));
}

#[test]
fn numeric_boundary_contains_i64_max() {
    let inputs = fz_corpus::structure::numeric_boundary_inputs();
    assert!(inputs.iter().any(|i| {
        let s = String::from_utf8_lossy(&i.data);
        s.contains("9223372036854775807")
    }));
}

#[test]
fn numeric_boundary_descriptions_non_empty() {
    let inputs = fz_corpus::structure::numeric_boundary_inputs();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn deep_nesting_non_empty() {
    let inputs = fz_corpus::structure::deep_nesting(3);
    assert!(!inputs.is_empty());
}

#[test]
fn deep_nesting_contains_nested_parens() {
    let inputs = fz_corpus::structure::deep_nesting(2);
    assert!(inputs.iter().any(|i| i.data == b"(())"));
}

#[test]
fn deep_nesting_depth_affects_size() {
    let shallow = fz_corpus::structure::deep_nesting(1);
    let deep = fz_corpus::structure::deep_nesting(5);
    let shallow_max: usize = shallow.iter().map(|i| i.data.len()).max().unwrap();
    let deep_max: usize = deep.iter().map(|i| i.data.len()).max().unwrap();
    assert!(deep_max > shallow_max);
}

#[test]
fn deep_nesting_handles_zero_depth() {
    let inputs = fz_corpus::structure::deep_nesting(0);
    assert!(!inputs.is_empty());
}

#[test]
fn deep_nesting_descriptions_non_empty() {
    let inputs = fz_corpus::structure::deep_nesting(2);
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}

#[test]
fn weird_id_non_empty() {
    let inputs = fz_corpus::text::weird_identifiers();
    assert!(!inputs.is_empty());
}

#[test]
fn weird_id_contains_empty() {
    let inputs = fz_corpus::text::weird_identifiers();
    assert!(inputs.iter().any(|i| i.data.is_empty()));
}

#[test]
fn weird_id_contains_keyword() {
    let inputs = fz_corpus::text::weird_identifiers();
    assert!(inputs.iter().any(|i| {
        let s = String::from_utf8_lossy(&i.data);
        s.contains("for") || s.contains("if") || s.contains("while")
    }));
}

#[test]
fn weird_id_contains_underscore() {
    let inputs = fz_corpus::text::weird_identifiers();
    assert!(inputs.iter().any(|i| i.data.contains(&b'_')));
}

#[test]
fn weird_id_contains_very_long() {
    let inputs = fz_corpus::text::weird_identifiers();
    assert!(inputs.iter().any(|i| i.data.len() > 100));
}

#[test]
fn weird_id_descriptions_non_empty() {
    let inputs = fz_corpus::text::weird_identifiers();
    for input in &inputs {
        assert!(!input.description.is_empty());
    }
}
