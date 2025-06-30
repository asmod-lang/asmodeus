use crate::formatter::core::format_assembly_content;

// ==========================
// HEADER AND COMMENT TESTS
// ==========================

#[test]
fn test_header_comments_no_indent() {
    let input = r#"; example: Test file
; This is a header comment
; Another header comment

start:
    POB #42
    STP"#;

    let expected = r#"; example: Test file
; This is a header comment
; Another header comment

start:
    POB #42
    STP
"#;

    let result = format_assembly_content(input);
    assert_eq!(result, expected);
}

#[test]
fn test_inline_comments_with_indent() {
    let input = r#"start:
    POB #42    ; this comment should be indented
    STP        ; this too"#;

    let expected = r#"start:
    POB #42              ; this comment should be indented
    STP                  ; this too

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_comment_space_after_semicolon() {
    let input = r#"start:
    POB #42;no space after semicolon
    STP     ;also no space"#;

    let expected = r#"start:
    POB #42              ; no space after semicolon
    STP                  ; also no space

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

// ============
// LABEL TESTS
// ============

#[test]
fn test_label_instruction_separation() {
    let input = r#"start:POB first
end:STP"#;

    let expected = r#"start:
    POB first
end:
    STP

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_label_instruction_with_comment_separation() {
    let input = r#"start:POB first;load value
loop:DOD second   ; add value"#;

    let expected = r#"start:
    POB first            ; load value
loop:
    DOD second           ; add value

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_labels_always_left_aligned() {
    let input = r#"    start:
        loop_label:
            end_label:"#;

    let expected = r#"start:
loop_label:
end_label:

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_simple_data_labels() {
    let input = r#"start:
    POB value
    STP

value:RST 42
message:   RST 100
buffer: RPA"#;

    let expected = r#"start:
    POB value
    STP

value: RST 42
message: RST 100
buffer: RPA

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

// ==================
// INSTRUCTION TESTS
// ==================

#[test]
fn test_instruction_formatting() {
    let input = r#"start:
POB    first
DOD      second
    WYJSCIE
        STP"#;

    let expected = r#"start:
    POB first
    DOD second
    WYJSCIE
    STP

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_directives() {
    let input = r#".data
    .text
.section    .rodata"#;

    let expected = r#"    .data
    .text
    .section .rodata

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

// ============
// ARRAY TESTS
// ============

#[test]
fn test_array_with_commas() {
    let input = r#"array: RST 10,20,30,40"#;

    let expected = r#"array: RST 10
       RST 20
       RST 30
       RST 40

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_multiline_array() {
    let input = r#"data_array: RST 10
RST 20
RST 30
RST 40
next_label: RST 50"#;

    let expected = r#"data_array: RST 10
            RST 20
            RST 30
            RST 40
next_label: RST 50

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_mixed_arrays_and_labels() {
    let input = r#"first: RST 1
second: RST 2,3,4
third: RST 5
fourth: RST 6
RST 7
RST 8"#;

    let expected = r#"first: RST 1
second: RST 2
        RST 3
        RST 4
third: RST 5
fourth: RST 6
        RST 7
        RST 8

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

// ===============================
// WHITESPACE AND STRUCTURE TESTS
// ===============================

#[test]
fn test_empty_lines_handling() {
    let input = r#"; header


start:


    POB #42


    STP


"#;

    let expected = r#"; header


start:


    POB #42


    STP

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_edge_cases() {
    // empty file
    assert_eq!(format_assembly_content(""), "\n");

    // only comments
    assert_eq!(format_assembly_content("; only comment"), "; only comment\n");

    // just label
    assert_eq!(format_assembly_content("start:"), "start:\n");

    // just instruction
    assert_eq!(format_assembly_content("POB #42"), "    POB #42\n");
}

// ==========================
// COMPLEX INTEGRATION TESTS
// ==========================

#[test]
fn test_complex_program() {
    let input = r#"; example: Complex test program
; Tests multiple formatting features

    start:POB first;load first
DOD    second ; add second
    WYJSCIE
STP

loop:    POB counter
SOZ end
    DOD #1
LAD counter
SOB loop

end:STP

first:RST 25
second:    RST 17
array: RST 1,2,3
buffer:RPA
multiline: RST 10
RST 20
RST 30"#;

    let expected = r#"; example: Complex test program
; Tests multiple formatting features

start:
    POB first            ; load first
    DOD second           ; add second
    WYJSCIE
    STP

loop:
    POB counter
    SOZ end
    DOD #1
    LAD counter
    SOB loop

end:
    STP

first: RST 25
second: RST 17
array: RST 1
       RST 2
       RST 3
buffer: RPA
multiline: RST 10
           RST 20
           RST 30

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

// ===========================
// INDIVIDUAL COMPONENT TESTS 
// ===========================

#[test]
fn test_various_instruction_formats() {
    let input = r#"start:
    POB    #42
    DOD   arg1   arg2
    MAKRO   name   param1   param2
    WYJSCIE
    STP"#;

    let expected = r#"start:
    POB #42
    DOD arg1 arg2
    MAKRO name param1 param2
    WYJSCIE
    STP

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_data_with_different_formats() {
    let input = r#"decimal: RST 42
hex_val: RST 0x2A
negative: RST -10
buffer: RPA"#;

    let expected = r#"decimal: RST 42
hex_val: RST 0x2A
negative: RST -10
buffer: RPA

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_comments_with_data() {
    let input = r#"value: RST 42;this is data
buffer: RPA   ; buffer space
array: RST 1,2,3   ;array data"#;

    let expected = r#"value: RST 42        ; this is data
buffer: RPA          ; buffer space
array: RST 1         ; array data
       RST 2
       RST 3

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_mixed_label_types() {
    let input = r#"standalone:
attached:POB #42
data_label: RST 100
array_label: RST 1,2,3"#;

    let expected = r#"standalone:
attached:
    POB #42
data_label: RST 100
array_label: RST 1
             RST 2
             RST 3

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

// =================
// REGRESSION TESTS
// =================

#[test]
fn test_preserve_important_spacing() {
    let input = r#"; header comment

start:
    POB #42

    STP

; footer comment"#;

    let expected = r#"; header comment

start:
    POB #42

    STP

    ; footer comment

"#;

    let result = format_assembly_content(input);
    assert_eq!(result.trim_end(), expected.trim_end());
}

#[test]
fn test_no_extra_spaces_in_output() {
    let input = r#"start:
    POB #42
    STP"#;

    let result = format_assembly_content(input);
    
    // no line has trailing spaces
    for line in result.lines() {
        assert_eq!(line, line.trim_end(), "Line should not have trailing spaces: '{}'", line);
    }
}
