//! The tests of the lexer, grouped by the rules of the specifications.

use super::{Keyword as K, LexError, LexErrorKind as E, Token, TokenKind as T, tokenize};
use crate::span::{Position, Span};

/// The kinds of the tokens of `source`, which must have no error.
fn kinds(source: &str) -> Vec<T> {
    tokenize(source)
        .unwrap()
        .into_iter()
        .map(|token| token.kind)
        .collect()
}

/// The tokens of `source`, which must have no error.
fn tokens(source: &str) -> Vec<Token> {
    tokenize(source).unwrap()
}

/// The `n`-th token of `source` (from 0) of the kind `kind`, which must have no error.
fn nth(source: &str, kind: &T, n: usize) -> Token {
    tokens(source)
        .into_iter()
        .filter(|token| token.kind == *kind)
        .nth(n)
        .unwrap()
}

/// The first token of `source` of the kind `kind`, which must have no error.
fn first(source: &str, kind: &T) -> Token {
    nth(source, kind, 0)
}

/// The error of `source`, which must have one.
fn error(source: &str) -> LexError {
    tokenize(source).unwrap_err()
}

/// The kind of the error of `source`, which must have one.
fn error_kind(source: &str) -> E {
    error(source).kind
}

fn name(word: &str) -> T {
    T::Name(word.to_owned())
}

fn text(content: &str) -> T {
    T::Text(content.to_owned())
}

/// The stretch of `line` from column `start` included to `end` excluded.
fn span(line: u32, start: u32, end: u32) -> Span {
    Span::new(Position::new(line, start), Position::new(line, end))
}

/// The empty stretch at `line` and `column`.
fn at(line: u32, column: u32) -> Span {
    Span::at(Position::new(line, column))
}

fn token(kind: T, span: Span) -> Token {
    Token { kind, span }
}

/// The kinds of a single line `source`, without its `Newline` and `End`.
fn line_kinds(source: &str) -> Vec<T> {
    let mut kinds = kinds(source);
    assert_eq!(kinds.pop(), Some(T::End));
    assert_eq!(kinds.pop(), Some(T::Newline));
    kinds
}

mod source {
    use super::*;

    #[test]
    fn an_empty_source_gives_only_the_end() {
        assert_eq!(tokens(""), vec![token(T::End, at(1, 1))]);
    }

    #[test]
    fn a_source_of_empty_lines_gives_only_the_end() {
        assert_eq!(kinds("\n\n\n"), vec![T::End]);
    }

    #[test]
    fn a_source_of_spaces_gives_only_the_end() {
        assert_eq!(kinds("    \n  \n        "), vec![T::End]);
    }

    #[test]
    fn a_source_of_comments_gives_only_the_end() {
        assert_eq!(kinds("# one\n    # two\n#"), vec![T::End]);
    }

    #[test]
    fn a_statement_ends_with_a_newline_then_the_end() {
        assert_eq!(
            kinds("forward()"),
            vec![
                name("forward"),
                T::LeftParen,
                T::RightParen,
                T::Newline,
                T::End
            ]
        );
    }

    #[test]
    fn a_final_line_break_adds_no_newline() {
        assert_eq!(kinds("forward()\n"), kinds("forward()"));
    }

    #[test]
    fn several_final_line_breaks_add_no_newline() {
        assert_eq!(kinds("forward()\n\n\n"), kinds("forward()"));
    }

    #[test]
    fn each_line_gives_its_own_newline() {
        assert_eq!(
            kinds("forward()\nturn_left()"),
            vec![
                name("forward"),
                T::LeftParen,
                T::RightParen,
                T::Newline,
                name("turn_left"),
                T::LeftParen,
                T::RightParen,
                T::Newline,
                T::End
            ]
        );
    }

    #[test]
    fn the_end_is_after_the_last_character() {
        assert_eq!(tokens("ab").last(), Some(&token(T::End, at(1, 3))));
    }

    #[test]
    fn the_end_is_on_the_last_line_after_a_final_line_break() {
        assert_eq!(tokens("ab\n").last(), Some(&token(T::End, at(2, 1))));
    }

    #[test]
    fn the_end_is_after_a_final_comment() {
        assert_eq!(tokens("a\n# done").last(), Some(&token(T::End, at(2, 7))));
    }

    #[test]
    fn the_end_comes_once_and_last() {
        let kinds = kinds("if x:\n    a\nb\n");
        assert_eq!(kinds.iter().filter(|kind| **kind == T::End).count(), 1);
        assert_eq!(kinds.last(), Some(&T::End));
    }
}

mod lines {
    use super::*;

    #[test]
    fn crlf_line_breaks_read_as_lf() {
        assert_eq!(
            tokens("if x:\r\n    a\r\nb\r\n"),
            tokens("if x:\n    a\nb\n")
        );
    }

    #[test]
    fn a_crlf_line_ends_before_its_carriage_return() {
        assert_eq!(first("ab\r\n", &T::Newline), token(T::Newline, at(1, 3)));
        assert_eq!(tokens("ab\r\n").last(), Some(&token(T::End, at(2, 1))));
    }

    #[test]
    fn a_lone_carriage_return_is_unexpected() {
        assert_eq!(
            error("a\rb"),
            LexError {
                kind: E::UnexpectedCharacter { character: '\r' },
                span: span(1, 2, 3)
            }
        );
    }

    #[test]
    fn a_line_cannot_be_split_inside_parentheses() {
        assert_eq!(
            kinds("print(1,\n2)"),
            vec![
                name("print"),
                T::LeftParen,
                T::Number(1),
                T::Comma,
                T::Newline,
                T::Number(2),
                T::RightParen,
                T::Newline,
                T::End
            ]
        );
    }

    #[test]
    fn a_line_continued_under_its_parenthesis_is_a_wrong_indentation() {
        assert_eq!(
            error_kind("print(1,\n      2)"),
            E::WrongIndent {
                found: 6,
                below: 0,
                above: 4
            }
        );
    }

    #[test]
    fn a_backslash_does_not_join_two_lines() {
        assert_eq!(
            error("let x = 1 + \\\n2"),
            LexError {
                kind: E::UnexpectedCharacter { character: '\\' },
                span: span(1, 13, 14)
            }
        );
    }

    #[test]
    fn the_newline_is_just_after_the_last_token() {
        assert_eq!(
            first("forward()", &T::Newline),
            token(T::Newline, at(1, 10))
        );
    }

    #[test]
    fn the_newline_ignores_the_trailing_spaces() {
        assert_eq!(
            first("forward()    ", &T::Newline),
            token(T::Newline, at(1, 10))
        );
    }

    #[test]
    fn the_newline_ignores_a_trailing_comment() {
        assert_eq!(
            first("if x # here", &T::Newline),
            token(T::Newline, at(1, 5))
        );
    }

    #[test]
    fn the_newline_follows_an_indented_line() {
        assert_eq!(
            nth("if x:\n    a", &T::Newline, 1),
            token(T::Newline, at(2, 6))
        );
    }

    #[test]
    fn the_lines_are_numbered_from_one() {
        let source = "a\nb\n\nc";
        assert_eq!(first(source, &name("a")).span, span(1, 1, 2));
        assert_eq!(first(source, &name("b")).span, span(2, 1, 2));
        assert_eq!(first(source, &name("c")).span, span(4, 1, 2));
    }
}

mod indentation {
    use super::*;

    #[test]
    fn four_more_spaces_open_a_block_and_going_back_closes_it() {
        assert_eq!(
            kinds("if x:\n    a\nb"),
            vec![
                T::Keyword(K::If),
                name("x"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("a"),
                T::Newline,
                T::Dedent,
                name("b"),
                T::Newline,
                T::End
            ]
        );
    }

    #[test]
    fn a_block_still_open_is_closed_at_the_end() {
        assert_eq!(
            kinds("if x:\n    a"),
            vec![
                T::Keyword(K::If),
                name("x"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("a"),
                T::Newline,
                T::Dedent,
                T::End
            ]
        );
    }

    #[test]
    fn a_block_still_open_is_closed_after_a_final_line_break() {
        assert_eq!(kinds("if x:\n    a\n"), kinds("if x:\n    a"));
    }

    #[test]
    fn the_blocks_still_open_are_closed_at_the_end_of_the_source() {
        let tokens = tokens("a:\n    b:\n        c\n");
        let end = at(4, 1);
        assert_eq!(
            tokens[tokens.len() - 3..],
            [
                token(T::Dedent, end),
                token(T::Dedent, end),
                token(T::End, end)
            ]
        );
    }

    #[test]
    fn the_indent_covers_the_indentation() {
        assert_eq!(
            first("a:\n    b", &T::Indent),
            token(T::Indent, span(2, 1, 5))
        );
    }

    #[test]
    fn a_deeper_indent_covers_its_whole_indentation() {
        assert_eq!(
            nth("a:\n    b:\n        c", &T::Indent, 1),
            token(T::Indent, span(3, 1, 9))
        );
    }

    #[test]
    fn the_dedent_is_before_the_first_token_of_its_line() {
        assert_eq!(
            first("a:\n    b\nc", &T::Dedent),
            token(T::Dedent, at(3, 1))
        );
    }

    #[test]
    fn the_dedent_to_an_inner_level_is_before_the_first_token() {
        let source = "a:\n    b:\n        c\n    d";
        assert_eq!(first(source, &T::Dedent), token(T::Dedent, at(4, 5)));
        assert_eq!(first(source, &name("d")).span, span(4, 5, 6));
    }

    #[test]
    fn nested_blocks_close_one_dedent_each() {
        assert_eq!(
            kinds("a:\n    b:\n        c:\n            d\ne"),
            vec![
                name("a"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("b"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("c"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("d"),
                T::Newline,
                T::Dedent,
                T::Dedent,
                T::Dedent,
                name("e"),
                T::Newline,
                T::End
            ]
        );
    }

    #[test]
    fn going_back_to_a_middle_level_closes_only_the_inner_blocks() {
        assert_eq!(
            kinds("a:\n    b:\n        c\n    d"),
            vec![
                name("a"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("b"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("c"),
                T::Newline,
                T::Dedent,
                name("d"),
                T::Newline,
                T::Dedent,
                T::End
            ]
        );
    }

    #[test]
    fn two_blocks_side_by_side() {
        assert_eq!(
            kinds("a:\n    b\nc:\n    d"),
            vec![
                name("a"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("b"),
                T::Newline,
                T::Dedent,
                name("c"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("d"),
                T::Newline,
                T::Dedent,
                T::End
            ]
        );
    }

    #[test]
    fn lines_at_the_same_level_open_nothing() {
        assert_eq!(
            kinds("a:\n    b\n    c"),
            vec![
                name("a"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("b"),
                T::Newline,
                name("c"),
                T::Newline,
                T::Dedent,
                T::End
            ]
        );
    }

    #[test]
    fn a_block_opens_again_after_a_dedent() {
        let kinds = kinds("a:\n    b:\n        c\n    d:\n        e");
        assert_eq!(kinds.iter().filter(|kind| **kind == T::Indent).count(), 3);
        assert_eq!(kinds.iter().filter(|kind| **kind == T::Dedent).count(), 3);
    }

    #[test]
    fn ten_nested_blocks_close_at_once() {
        let mut source = String::new();
        for depth in 0..10 {
            source.push_str(&" ".repeat(depth * 4));
            source.push_str("a:\n");
        }
        source.push_str(&" ".repeat(40));
        source.push_str("b\nc");
        let kinds = kinds(&source);
        assert_eq!(kinds.iter().filter(|kind| **kind == T::Indent).count(), 10);
        let dedents = kinds.iter().skip_while(|kind| **kind != name("b"));
        assert_eq!(dedents.filter(|kind| **kind == T::Dedent).count(), 10);
    }

    #[test]
    fn the_lexer_indents_a_first_line_and_leaves_its_error_to_the_parser() {
        assert_eq!(
            kinds("    a"),
            vec![T::Indent, name("a"), T::Newline, T::Dedent, T::End]
        );
    }

    #[test]
    fn the_lexer_indents_a_line_without_colon_and_leaves_its_error_to_the_parser() {
        assert_eq!(
            kinds("if x\n    a"),
            vec![
                T::Keyword(K::If),
                name("x"),
                T::Newline,
                T::Indent,
                name("a"),
                T::Newline,
                T::Dedent,
                T::End
            ]
        );
    }

    #[test]
    fn the_lexer_leaves_an_empty_block_to_the_parser() {
        assert_eq!(
            kinds("if x:\na"),
            vec![
                T::Keyword(K::If),
                name("x"),
                T::Colon,
                T::Newline,
                name("a"),
                T::Newline,
                T::End
            ]
        );
    }

    #[test]
    fn two_spaces_more_are_a_wrong_indentation() {
        assert_eq!(
            error("a:\n  b"),
            LexError {
                kind: E::WrongIndent {
                    found: 2,
                    below: 0,
                    above: 4
                },
                span: span(2, 1, 3)
            }
        );
    }

    #[test]
    fn one_space_more_is_a_wrong_indentation() {
        assert_eq!(
            error_kind("a:\n b"),
            E::WrongIndent {
                found: 1,
                below: 0,
                above: 4
            }
        );
    }

    #[test]
    fn three_spaces_more_are_a_wrong_indentation() {
        assert_eq!(
            error_kind("a:\n   b"),
            E::WrongIndent {
                found: 3,
                below: 0,
                above: 4
            }
        );
    }

    #[test]
    fn five_spaces_more_are_a_wrong_indentation() {
        assert_eq!(
            error_kind("a:\n     b"),
            E::WrongIndent {
                found: 5,
                below: 0,
                above: 4
            }
        );
    }

    #[test]
    fn eight_spaces_more_are_a_wrong_indentation() {
        assert_eq!(
            error("a:\n        b"),
            LexError {
                kind: E::WrongIndent {
                    found: 8,
                    below: 0,
                    above: 4
                },
                span: span(2, 1, 9)
            }
        );
    }

    #[test]
    fn a_wrong_indentation_inside_a_block_counts_from_its_level() {
        assert_eq!(
            error_kind("a:\n    b:\n          c"),
            E::WrongIndent {
                found: 10,
                below: 4,
                above: 8
            }
        );
    }

    #[test]
    fn going_back_between_two_levels_is_a_wrong_indentation() {
        assert_eq!(
            error("a:\n    b:\n        c\n      d"),
            LexError {
                kind: E::WrongIndent {
                    found: 6,
                    below: 4,
                    above: 8
                },
                span: span(4, 1, 7)
            }
        );
    }

    #[test]
    fn going_back_below_the_first_block_is_a_wrong_indentation() {
        assert_eq!(
            error_kind("a:\n    b\n  c"),
            E::WrongIndent {
                found: 2,
                below: 0,
                above: 4
            }
        );
    }

    #[test]
    fn going_back_far_between_two_levels_names_the_two_closest() {
        assert_eq!(
            error_kind("a:\n    b:\n        c\n d"),
            E::WrongIndent {
                found: 1,
                below: 0,
                above: 4
            }
        );
    }

    #[test]
    fn a_wrong_indentation_stops_at_its_line() {
        assert_eq!(error("a:\n  b\n\t").span.start.line, 2);
    }

    #[test]
    fn a_tab_as_indentation_is_an_error() {
        assert_eq!(
            error("if x:\n\tforward()"),
            LexError {
                kind: E::Tab,
                span: span(2, 1, 2)
            }
        );
    }

    #[test]
    fn a_tab_after_spaces_is_an_error() {
        assert_eq!(
            error("if x:\n    \tforward()"),
            LexError {
                kind: E::Tab,
                span: span(2, 5, 6)
            }
        );
    }

    #[test]
    fn a_tab_before_spaces_is_an_error() {
        assert_eq!(
            error("if x:\n\t    forward()"),
            LexError {
                kind: E::Tab,
                span: span(2, 1, 2)
            }
        );
    }

    #[test]
    fn the_first_of_several_tabs_is_the_error() {
        assert_eq!(error("  \t \tforward()").span, span(1, 3, 4));
    }

    #[test]
    fn a_tab_on_the_first_line_is_an_error() {
        assert_eq!(error_kind("\tforward()"), E::Tab);
    }

    #[test]
    fn a_tab_between_two_tokens_is_an_error() {
        assert_eq!(
            error("let\tx = 1"),
            LexError {
                kind: E::Tab,
                span: span(1, 4, 5)
            }
        );
    }

    #[test]
    fn a_tab_at_the_end_of_a_line_is_an_error() {
        assert_eq!(error("forward()\t").span, span(1, 10, 11));
    }

    #[test]
    fn a_tab_before_a_comment_is_an_error() {
        assert_eq!(error_kind("forward()\t# go"), E::Tab);
    }

    #[test]
    fn a_tab_inside_a_text_is_kept() {
        assert_eq!(
            line_kinds("print(\"a\tb\")"),
            vec![name("print"), T::LeftParen, text("a\tb"), T::RightParen]
        );
    }

    #[test]
    fn a_tab_inside_a_comment_is_ignored() {
        assert_eq!(kinds("forward() #\tgo"), kinds("forward()"));
    }

    #[test]
    fn a_line_of_tabs_is_ignored() {
        assert_eq!(kinds("a:\n    b\n\t\t\n    c"), kinds("a:\n    b\n    c"));
    }

    #[test]
    fn a_comment_indented_by_a_tab_is_ignored() {
        assert_eq!(kinds("a:\n    b\n\t# c\n    d"), kinds("a:\n    b\n    d"));
    }

    #[test]
    fn an_empty_line_inside_a_block_does_not_close_it() {
        assert_eq!(kinds("a:\n    b\n\n    c"), kinds("a:\n    b\n    c"));
    }

    #[test]
    fn a_line_of_spaces_inside_a_block_does_not_close_it() {
        assert_eq!(kinds("a:\n    b\n  \n    c"), kinds("a:\n    b\n    c"));
    }

    #[test]
    fn a_line_of_many_spaces_is_ignored() {
        assert_eq!(kinds("a\n                 \nb"), kinds("a\nb"));
    }

    #[test]
    fn comments_at_any_indentation_are_ignored() {
        assert_eq!(
            kinds("a:\n    b\n# c\n  # d\n            # e\n    f"),
            kinds("a:\n    b\n    f")
        );
    }

    #[test]
    fn an_empty_line_before_a_dedent_keeps_it_on_the_next_line() {
        assert_eq!(
            first("a:\n    b\n\nc", &T::Dedent),
            token(T::Dedent, at(4, 1))
        );
    }
}

mod comments {
    use super::*;

    #[test]
    fn a_comment_after_a_statement_is_ignored() {
        assert_eq!(kinds("forward() # go"), kinds("forward()"));
    }

    #[test]
    fn a_comment_right_after_a_token_is_ignored() {
        assert_eq!(kinds("forward()#go"), kinds("forward()"));
    }

    #[test]
    fn a_comment_may_hold_anything() {
        assert_eq!(kinds("a # \"' @ ; \\ ` é 🎉 #"), kinds("a"));
    }

    #[test]
    fn a_hash_inside_a_text_starts_no_comment() {
        assert_eq!(
            line_kinds("print(\"#1\")"),
            vec![name("print"), T::LeftParen, text("#1"), T::RightParen]
        );
    }

    #[test]
    fn a_comment_after_a_text_with_a_hash() {
        assert_eq!(
            line_kinds("print(\"#\") # print a hash"),
            vec![name("print"), T::LeftParen, text("#"), T::RightParen]
        );
    }

    #[test]
    fn an_empty_comment_is_ignored() {
        assert_eq!(kinds("a #"), kinds("a"));
    }
}

mod names {
    use super::*;

    #[test]
    fn snake_case_names() {
        assert_eq!(
            line_kinds("sides paint_side count_2"),
            vec![name("sides"), name("paint_side"), name("count_2")]
        );
    }

    #[test]
    fn a_single_letter_is_a_name() {
        assert_eq!(line_kinds("x"), vec![name("x")]);
    }

    #[test]
    fn a_name_with_digits_inside() {
        assert_eq!(line_kinds("x2y3"), vec![name("x2y3")]);
    }

    #[test]
    fn a_name_with_double_and_final_underscores_is_one_name() {
        assert_eq!(line_kinds("a__b x_"), vec![name("a__b"), name("x_")]);
    }

    #[test]
    fn a_name_starting_with_an_underscore_is_left_to_the_checker() {
        assert_eq!(line_kinds("_x __"), vec![name("_x"), name("__")]);
    }

    #[test]
    fn camel_case_is_one_name_left_to_the_checker() {
        assert_eq!(line_kinds("paintSide"), vec![name("paintSide")]);
    }

    #[test]
    fn pascal_case_and_capitals_are_names_left_to_the_checker() {
        assert_eq!(
            line_kinds("PaintSide SIDES"),
            vec![name("PaintSide"), name("SIDES")]
        );
    }

    #[test]
    fn a_name_with_accents_is_one_name_left_to_the_checker() {
        assert_eq!(tokens("côté")[0], token(name("côté"), span(1, 1, 5)));
    }

    #[test]
    fn names_in_other_alphabets_are_left_to_the_checker() {
        assert_eq!(
            line_kinds("сторона πλευρά 辺"),
            vec![name("сторона"), name("πλευρά"), name("辺")]
        );
    }

    #[test]
    fn every_keyword_is_a_keyword() {
        for keyword in K::ALL {
            assert_eq!(
                line_kinds(keyword.as_str()),
                vec![T::Keyword(keyword)],
                "{keyword:?}"
            );
        }
    }

    #[test]
    fn the_keywords_of_the_specifications() {
        let words: Vec<_> = K::ALL.into_iter().map(K::as_str).collect();
        assert_eq!(
            words,
            [
                "let", "if", "else", "while", "repeat", "break", "function", "return", "and", "or",
                "not", "true", "false"
            ]
        );
    }

    #[test]
    fn every_keyword_reads_back_from_its_word() {
        for keyword in K::ALL {
            assert_eq!(K::from_word(keyword.as_str()), Some(keyword));
        }
    }

    #[test]
    fn a_name_is_no_keyword() {
        assert_eq!(K::from_word("sides"), None);
        assert_eq!(K::from_word(""), None);
    }

    #[test]
    fn keywords_are_lowercase_only() {
        assert_eq!(
            line_kinds("Let IF True FALSE"),
            vec![name("Let"), name("IF"), name("True"), name("FALSE")]
        );
    }

    #[test]
    fn a_word_that_starts_with_a_keyword_is_a_name() {
        assert_eq!(
            line_kinds("letter iffy nothing android order repeated true_count"),
            vec![
                name("letter"),
                name("iffy"),
                name("nothing"),
                name("android"),
                name("order"),
                name("repeated"),
                name("true_count")
            ]
        );
    }

    #[test]
    fn a_keyword_followed_by_a_parenthesis() {
        assert_eq!(
            line_kinds("if(x)"),
            vec![T::Keyword(K::If), T::LeftParen, name("x"), T::RightParen]
        );
    }

    #[test]
    fn the_words_of_other_languages_are_names_for_the_parser_and_the_checker() {
        for word in [
            "elif", "def", "var", "const", "for", "continue", "pass", "none", "null",
        ] {
            assert_eq!(line_kinds(word), vec![name(word)], "{word}");
        }
    }

    #[test]
    fn the_built_in_functions_and_the_colors_are_names() {
        assert_eq!(
            line_kinds("forward print red yellow blue"),
            vec![
                name("forward"),
                name("print"),
                name("red"),
                name("yellow"),
                name("blue")
            ]
        );
    }

    #[test]
    fn a_word_starting_with_a_digit_is_an_error() {
        assert_eq!(
            error("let 2nd_side = 3"),
            LexError {
                kind: E::NameStartsWithDigit {
                    word: "2nd_side".to_owned()
                },
                span: span(1, 5, 13)
            }
        );
    }

    #[test]
    fn a_digit_then_a_letter_is_an_error() {
        assert_eq!(
            error_kind("3x"),
            E::NameStartsWithDigit {
                word: "3x".to_owned()
            }
        );
    }

    #[test]
    fn a_digit_then_an_underscore_is_an_error() {
        assert_eq!(
            error_kind("1_000"),
            E::NameStartsWithDigit {
                word: "1_000".to_owned()
            }
        );
    }

    #[test]
    fn a_digit_then_an_accent_is_an_error() {
        assert_eq!(
            error("2é + 1"),
            LexError {
                kind: E::NameStartsWithDigit {
                    word: "2é".to_owned()
                },
                span: span(1, 1, 3)
            }
        );
    }

    #[test]
    fn a_name_span_counts_its_characters() {
        assert_eq!(
            tokens("let paint_side"),
            vec![
                token(T::Keyword(K::Let), span(1, 1, 4)),
                token(name("paint_side"), span(1, 5, 15)),
                token(T::Newline, at(1, 15)),
                token(T::End, at(1, 15)),
            ]
        );
    }
}

mod numbers {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(line_kinds("0"), vec![T::Number(0)]);
    }

    #[test]
    fn a_whole_number() {
        assert_eq!(tokens("42")[0], token(T::Number(42), span(1, 1, 3)));
    }

    #[test]
    fn leading_zeros_are_read() {
        assert_eq!(line_kinds("007"), vec![T::Number(7)]);
    }

    #[test]
    fn a_negative_number_is_a_minus_then_its_value() {
        assert_eq!(line_kinds("-7"), vec![T::Minus, T::Number(7)]);
    }

    #[test]
    fn a_minus_apart_from_its_number() {
        assert_eq!(line_kinds("- 7"), vec![T::Minus, T::Number(7)]);
    }

    #[test]
    fn the_largest_number() {
        assert_eq!(line_kinds("9223372036854775807"), vec![T::Number(i64::MAX)]);
    }

    #[test]
    fn one_more_than_the_largest_number_is_too_large() {
        assert_eq!(
            error("let x = 9223372036854775808"),
            LexError {
                kind: E::NumberTooLarge,
                span: span(1, 9, 28)
            }
        );
    }

    #[test]
    fn a_huge_number_is_too_large() {
        assert_eq!(error_kind(&"9".repeat(100)), E::NumberTooLarge);
    }

    #[test]
    fn the_smallest_number_is_too_large_written_as_a_literal() {
        // Its value without the sign doesn't fit: only computed, see Values in specs.md.
        assert_eq!(error_kind("-9223372036854775808"), E::NumberTooLarge);
    }

    #[test]
    fn a_number_followed_by_a_parenthesis() {
        assert_eq!(
            line_kinds("repeat 3:"),
            vec![T::Keyword(K::Repeat), T::Number(3), T::Colon]
        );
        assert_eq!(
            line_kinds("f(3)"),
            vec![name("f"), T::LeftParen, T::Number(3), T::RightParen]
        );
    }

    #[test]
    fn numbers_and_operators_without_spaces() {
        assert_eq!(
            line_kinds("1+2-3*4/5%6"),
            vec![
                T::Number(1),
                T::Plus,
                T::Number(2),
                T::Minus,
                T::Number(3),
                T::Star,
                T::Number(4),
                T::Slash,
                T::Number(5),
                T::Percent,
                T::Number(6)
            ]
        );
    }

    #[test]
    fn a_decimal_point_is_unexpected() {
        assert_eq!(
            error("1.5"),
            LexError {
                kind: E::UnexpectedCharacter { character: '.' },
                span: span(1, 2, 3)
            }
        );
    }

    #[test]
    fn a_digit_of_another_alphabet_alone_is_a_name_left_to_the_checker() {
        assert_eq!(line_kinds("٣"), vec![name("٣")]);
    }

    #[test]
    fn a_digit_of_another_alphabet_after_a_digit_is_a_name_starting_with_a_digit() {
        assert_eq!(
            error_kind("3٣"),
            E::NameStartsWithDigit {
                word: "3٣".to_owned()
            }
        );
    }
}

mod texts {
    use super::*;

    #[test]
    fn a_text() {
        assert_eq!(
            tokens("\"I am stuck\"")[0],
            token(text("I am stuck"), span(1, 1, 13))
        );
    }

    #[test]
    fn an_empty_text() {
        assert_eq!(tokens("\"\"")[0], token(text(""), span(1, 1, 3)));
    }

    #[test]
    fn a_text_of_spaces() {
        assert_eq!(line_kinds("\"   \""), vec![text("   ")]);
    }

    #[test]
    fn a_text_keeps_accents_and_emoji() {
        assert_eq!(
            line_kinds("\"Bravo, côté 🎉\""),
            vec![text("Bravo, côté 🎉")]
        );
    }

    #[test]
    fn a_text_keeps_a_single_quote() {
        assert_eq!(line_kinds("\"it's\""), vec![text("it's")]);
    }

    #[test]
    fn a_text_keeps_symbols_that_mean_nothing_outside() {
        assert_eq!(line_kinds("\"@;&|!`{}\""), vec![text("@;&|!`{}")]);
    }

    #[test]
    fn a_text_keeps_keywords_and_numbers() {
        assert_eq!(line_kinds("\"if 3\""), vec![text("if 3")]);
    }

    #[test]
    fn an_escaped_double_quote() {
        assert_eq!(line_kinds(r#""a\"b""#), vec![text("a\"b")]);
    }

    #[test]
    fn an_escaped_backslash() {
        assert_eq!(line_kinds(r#""a\\b""#), vec![text("a\\b")]);
    }

    #[test]
    fn an_escaped_line_break() {
        assert_eq!(line_kinds(r#""a\nb""#), vec![text("a\nb")]);
    }

    #[test]
    fn an_escaped_backslash_at_the_end_closes_the_text() {
        assert_eq!(line_kinds(r#""a\\""#), vec![text("a\\")]);
    }

    #[test]
    fn an_escaped_backslash_then_an_n() {
        assert_eq!(line_kinds(r#""\\n""#), vec![text("\\n")]);
    }

    #[test]
    fn several_escapes_in_a_row() {
        assert_eq!(line_kinds(r#""\"\\\n\"""#), vec![text("\"\\\n\"")]);
    }

    #[test]
    fn a_text_of_escaped_quotes() {
        assert_eq!(line_kinds(r#""\"\"""#), vec![text("\"\"")]);
    }

    #[test]
    fn the_span_of_a_text_covers_its_escapes_as_written() {
        assert_eq!(tokens(r#""a\nb""#)[0].span, span(1, 1, 7));
    }

    #[test]
    fn two_texts_on_a_line() {
        assert_eq!(
            line_kinds("print(\"Sides:\", \"four\")"),
            vec![
                name("print"),
                T::LeftParen,
                text("Sides:"),
                T::Comma,
                text("four"),
                T::RightParen
            ]
        );
    }

    #[test]
    fn two_texts_joined_without_space() {
        assert_eq!(line_kinds("\"a\"\"b\""), vec![text("a"), text("b")]);
    }

    #[test]
    fn a_text_not_closed_is_an_error_up_to_the_end_of_its_line() {
        assert_eq!(
            error("print(\"I am stuck)"),
            LexError {
                kind: E::UnclosedText,
                span: span(1, 7, 19)
            }
        );
    }

    #[test]
    fn a_text_not_closed_before_the_end_of_the_source() {
        assert_eq!(
            error("\""),
            LexError {
                kind: E::UnclosedText,
                span: span(1, 1, 2)
            }
        );
    }

    #[test]
    fn a_text_cannot_span_two_lines() {
        assert_eq!(
            error("print(\"one\ntwo\")"),
            LexError {
                kind: E::UnclosedText,
                span: span(1, 7, 11)
            }
        );
    }

    #[test]
    fn a_text_not_closed_ends_before_a_carriage_return() {
        assert_eq!(error("\"ab\r\n").span, span(1, 1, 4));
    }

    #[test]
    fn an_escaped_quote_does_not_close_a_text() {
        assert_eq!(
            error(r#""abc\""#),
            LexError {
                kind: E::UnclosedText,
                span: span(1, 1, 7)
            }
        );
    }

    #[test]
    fn a_backslash_at_the_end_of_a_line_does_not_close_a_text() {
        assert_eq!(
            error("\"abc\\\n\""),
            LexError {
                kind: E::UnclosedText,
                span: span(1, 1, 6)
            }
        );
    }

    #[test]
    fn a_hash_inside_a_text_not_closed_is_part_of_it() {
        assert_eq!(error("\"a # b").span, span(1, 1, 7));
    }

    #[test]
    fn an_unknown_escape_is_an_error_over_the_escape() {
        assert_eq!(
            error(r#"print("a\tb")"#),
            LexError {
                kind: E::UnknownEscape { escape: 't' },
                span: span(1, 9, 11)
            }
        );
    }

    #[test]
    fn an_escaped_single_quote_is_unknown() {
        assert_eq!(error_kind(r#""it\'s""#), E::UnknownEscape { escape: '\'' });
    }

    #[test]
    fn an_escaped_space_is_unknown() {
        assert_eq!(error_kind(r#""a\ b""#), E::UnknownEscape { escape: ' ' });
    }

    #[test]
    fn an_escaped_accent_is_unknown_and_counted_as_one_column() {
        assert_eq!(
            error(r#""\é""#),
            LexError {
                kind: E::UnknownEscape { escape: 'é' },
                span: span(1, 2, 4)
            }
        );
    }

    #[test]
    fn an_escaped_capital_n_is_unknown() {
        assert_eq!(error_kind(r#""\N""#), E::UnknownEscape { escape: 'N' });
    }

    #[test]
    fn the_first_unknown_escape_is_the_error() {
        assert_eq!(error(r#""\x\y""#).kind, E::UnknownEscape { escape: 'x' });
    }

    #[test]
    fn an_unknown_escape_comes_before_a_missing_quote() {
        assert_eq!(error_kind(r#""\t"#), E::UnknownEscape { escape: 't' });
    }

    #[test]
    fn a_single_quote_is_an_error() {
        assert_eq!(
            error("print('hello')"),
            LexError {
                kind: E::SingleQuote,
                span: span(1, 7, 8)
            }
        );
    }

    #[test]
    fn a_lone_single_quote_is_an_error() {
        assert_eq!(error_kind("'"), E::SingleQuote);
    }

    #[test]
    fn typographic_quotes_are_unexpected() {
        assert_eq!(
            error_kind("print(“hello”)"),
            E::UnexpectedCharacter { character: '“' }
        );
        assert_eq!(
            error_kind("print(«hello»)"),
            E::UnexpectedCharacter { character: '«' }
        );
    }
}

mod symbols {
    use super::*;

    #[test]
    fn the_punctuation() {
        assert_eq!(
            line_kinds("( ) , :"),
            vec![T::LeftParen, T::RightParen, T::Comma, T::Colon]
        );
    }

    #[test]
    fn the_arithmetic() {
        assert_eq!(
            line_kinds("+ - * / %"),
            vec![T::Plus, T::Minus, T::Star, T::Slash, T::Percent]
        );
    }

    #[test]
    fn the_comparisons() {
        assert_eq!(
            line_kinds("== != < <= > >="),
            vec![
                T::EqualEqual,
                T::NotEqual,
                T::Less,
                T::LessEqual,
                T::Greater,
                T::GreaterEqual
            ]
        );
    }

    #[test]
    fn the_assignment() {
        assert_eq!(
            line_kinds("let x = 3"),
            vec![T::Keyword(K::Let), name("x"), T::Equal, T::Number(3)]
        );
    }

    #[test]
    fn a_two_character_symbol_covers_both() {
        assert_eq!(
            first("a <= b", &T::LessEqual),
            token(T::LessEqual, span(1, 3, 5))
        );
        assert_eq!(
            first("a != b", &T::NotEqual),
            token(T::NotEqual, span(1, 3, 5))
        );
    }

    #[test]
    fn symbols_without_spaces() {
        assert_eq!(
            line_kinds("x==1"),
            vec![name("x"), T::EqualEqual, T::Number(1)]
        );
        assert_eq!(
            line_kinds("x>=-1"),
            vec![name("x"), T::GreaterEqual, T::Minus, T::Number(1)]
        );
    }

    #[test]
    fn the_longest_symbol_comes_first() {
        assert_eq!(line_kinds("==="), vec![T::EqualEqual, T::Equal]);
        assert_eq!(line_kinds("<=="), vec![T::LessEqual, T::Equal]);
        assert_eq!(line_kinds(">=="), vec![T::GreaterEqual, T::Equal]);
        assert_eq!(line_kinds("!=="), vec![T::NotEqual, T::Equal]);
    }

    #[test]
    fn symbols_in_the_wrong_order_are_two_tokens() {
        assert_eq!(line_kinds("=<"), vec![T::Equal, T::Less]);
        assert_eq!(line_kinds("=>"), vec![T::Equal, T::Greater]);
        assert_eq!(line_kinds("->"), vec![T::Minus, T::Greater]);
        assert_eq!(line_kinds("<>"), vec![T::Less, T::Greater]);
    }

    #[test]
    fn a_split_symbol_is_two_tokens() {
        assert_eq!(line_kinds("= ="), vec![T::Equal, T::Equal]);
        assert_eq!(line_kinds("< ="), vec![T::Less, T::Equal]);
    }

    #[test]
    fn symbols_of_other_languages_doubled_are_two_tokens_for_the_parser() {
        assert_eq!(line_kinds("**"), vec![T::Star, T::Star]);
        assert_eq!(line_kinds("//"), vec![T::Slash, T::Slash]);
        assert_eq!(line_kinds("++"), vec![T::Plus, T::Plus]);
        assert_eq!(line_kinds("+="), vec![T::Plus, T::Equal]);
    }

    #[test]
    fn the_logic_keywords() {
        assert_eq!(
            line_kinds("not a and b or c"),
            vec![
                T::Keyword(K::Not),
                name("a"),
                T::Keyword(K::And),
                name("b"),
                T::Keyword(K::Or),
                name("c")
            ]
        );
    }

    #[test]
    fn an_exclamation_mark_suggests_not() {
        assert_eq!(
            error("if !front_is_clear():"),
            LexError {
                kind: E::SymbolForKeyword {
                    symbol: "!",
                    keyword: K::Not
                },
                span: span(1, 4, 5)
            }
        );
    }

    #[test]
    fn an_exclamation_mark_apart_from_an_equal_suggests_not() {
        assert_eq!(
            error_kind("a ! = b"),
            E::SymbolForKeyword {
                symbol: "!",
                keyword: K::Not
            }
        );
    }

    #[test]
    fn a_double_ampersand_suggests_and() {
        assert_eq!(
            error("a && b"),
            LexError {
                kind: E::SymbolForKeyword {
                    symbol: "&&",
                    keyword: K::And
                },
                span: span(1, 3, 5)
            }
        );
    }

    #[test]
    fn an_ampersand_suggests_and() {
        assert_eq!(
            error("a & b"),
            LexError {
                kind: E::SymbolForKeyword {
                    symbol: "&",
                    keyword: K::And
                },
                span: span(1, 3, 4)
            }
        );
    }

    #[test]
    fn a_double_pipe_suggests_or() {
        assert_eq!(
            error("a || b"),
            LexError {
                kind: E::SymbolForKeyword {
                    symbol: "||",
                    keyword: K::Or
                },
                span: span(1, 3, 5)
            }
        );
    }

    #[test]
    fn a_pipe_suggests_or() {
        assert_eq!(
            error_kind("a | b"),
            E::SymbolForKeyword {
                symbol: "|",
                keyword: K::Or
            }
        );
    }

    #[test]
    fn three_ampersands_stop_at_the_first_two() {
        assert_eq!(error("a &&& b").span, span(1, 3, 5));
    }

    #[test]
    fn an_ampersand_then_a_pipe_stops_at_the_ampersand() {
        assert_eq!(
            error("a &| b"),
            LexError {
                kind: E::SymbolForKeyword {
                    symbol: "&",
                    keyword: K::And
                },
                span: span(1, 3, 4)
            }
        );
    }

    #[test]
    fn the_characters_without_meaning_are_unexpected() {
        for character in [
            '@', '$', ';', '.', '[', ']', '{', '}', '?', '~', '^', '`', '\\', '\u{a0}', '–',
            '\u{feff}', '\u{200b}', '\0', '\u{7}',
        ] {
            assert_eq!(
                error(&format!("a {character} b")),
                LexError {
                    kind: E::UnexpectedCharacter { character },
                    span: span(1, 3, 4)
                },
                "{character:?}"
            );
        }
    }

    #[test]
    fn a_non_breaking_space_as_indentation_is_unexpected() {
        assert_eq!(
            error("a:\n\u{a0}\u{a0}\u{a0}\u{a0}b"),
            LexError {
                kind: E::UnexpectedCharacter {
                    character: '\u{a0}'
                },
                span: span(2, 1, 2)
            }
        );
    }

    #[test]
    fn a_line_of_non_breaking_spaces_is_ignored() {
        assert_eq!(
            kinds("a:\n    b\n\u{a0}\u{a0}\n    c"),
            kinds("a:\n    b\n    c")
        );
    }

    #[test]
    fn a_line_of_blank_characters_of_any_kind_is_ignored() {
        assert_eq!(kinds("a\n \t\u{a0}\u{2003}\u{3000}\nb"), kinds("a\nb"));
    }

    #[test]
    fn a_comment_indented_by_non_breaking_spaces_is_ignored() {
        assert_eq!(
            kinds("a:\n    b\n\u{a0}# c\n    d"),
            kinds("a:\n    b\n    d")
        );
    }

    #[test]
    fn a_non_breaking_space_after_spaces_in_the_indentation_is_unexpected() {
        assert_eq!(
            error("a:\n    \u{a0}b"),
            LexError {
                kind: E::UnexpectedCharacter {
                    character: '\u{a0}'
                },
                span: span(2, 5, 6)
            }
        );
    }

    #[test]
    fn an_unexpected_emoji_is_one_column() {
        assert_eq!(error("🎉").span, span(1, 1, 2));
    }
}

mod errors {
    use super::*;

    #[test]
    fn the_first_error_stops_the_lexer() {
        assert_eq!(
            error("a @ b\n\tc\n\"d"),
            LexError {
                kind: E::UnexpectedCharacter { character: '@' },
                span: span(1, 3, 4)
            }
        );
    }

    #[test]
    fn an_error_on_a_later_line_keeps_its_line() {
        assert_eq!(error("a\nb\nc ;").span, span(3, 3, 4));
    }

    #[test]
    fn an_error_after_accents_counts_characters_not_bytes() {
        assert_eq!(error("print(\"éèà\") @").span, span(1, 14, 15));
    }

    #[test]
    fn an_error_after_an_emoji_counts_it_as_one_column() {
        assert_eq!(error("print(\"🎉\") @").span, span(1, 12, 13));
    }

    #[test]
    fn an_error_inside_a_block_keeps_its_column() {
        assert_eq!(error("a:\n    b @").span, span(2, 7, 8));
    }

    #[test]
    fn an_error_inside_a_comment_is_none() {
        assert!(tokenize("# @ ; \t ' \"").is_ok());
    }
}

mod positions {
    use super::*;

    #[test]
    fn the_spans_of_a_declaration() {
        assert_eq!(
            tokens("let x = 42"),
            vec![
                token(T::Keyword(K::Let), span(1, 1, 4)),
                token(name("x"), span(1, 5, 6)),
                token(T::Equal, span(1, 7, 8)),
                token(T::Number(42), span(1, 9, 11)),
                token(T::Newline, at(1, 11)),
                token(T::End, at(1, 11)),
            ]
        );
    }

    #[test]
    fn the_spans_of_a_call_with_a_text() {
        assert_eq!(
            tokens("print(\"é\", x)"),
            vec![
                token(name("print"), span(1, 1, 6)),
                token(T::LeftParen, span(1, 6, 7)),
                token(text("é"), span(1, 7, 10)),
                token(T::Comma, span(1, 10, 11)),
                token(name("x"), span(1, 12, 13)),
                token(T::RightParen, span(1, 13, 14)),
                token(T::Newline, at(1, 14)),
                token(T::End, at(1, 14)),
            ]
        );
    }

    #[test]
    fn the_spans_of_a_block() {
        assert_eq!(
            tokens("repeat 2:\n    forward()\n"),
            vec![
                token(T::Keyword(K::Repeat), span(1, 1, 7)),
                token(T::Number(2), span(1, 8, 9)),
                token(T::Colon, span(1, 9, 10)),
                token(T::Newline, at(1, 10)),
                token(T::Indent, span(2, 1, 5)),
                token(name("forward"), span(2, 5, 12)),
                token(T::LeftParen, span(2, 12, 13)),
                token(T::RightParen, span(2, 13, 14)),
                token(T::Newline, at(2, 14)),
                token(T::Dedent, at(3, 1)),
                token(T::End, at(3, 1)),
            ]
        );
    }

    #[test]
    fn the_spans_after_an_emoji_count_it_as_one_column() {
        assert_eq!(
            first("\"🎉\" + x", &name("x")),
            token(name("x"), span(1, 7, 8))
        );
    }

    #[test]
    fn the_tokens_follow_each_other() {
        let source = "let sides = 0\nwhile sides < 4:\n    paint_side(3)\n    sides = sides + 1\n";
        let tokens = tokens(source);
        for pair in tokens.windows(2) {
            assert!(pair[0].span.end <= pair[1].span.start, "{pair:?}");
        }
        for token in &tokens {
            assert!(token.span.start <= token.span.end, "{token:?}");
        }
    }
}

mod programs {
    use super::*;

    /// The example of the specifications, in *The language*.
    const EXAMPLE: &str = r#"# Paint a square of side 3
function paint_side(length):
    start_painting(red)
    repeat length:
        forward()
    stop_painting()

let sides = 0
while sides < 4:
    paint_side(3)
    turn_right()
    sides = sides + 1

if front_is_clear():
    forward()
else if left_is_clear():
    turn_left()
else:
    print("I am stuck")
"#;

    #[test]
    fn the_example_of_the_specifications() {
        let kinds = kinds(EXAMPLE);
        assert_eq!(kinds.iter().filter(|kind| **kind == T::Indent).count(), 6);
        assert_eq!(kinds.iter().filter(|kind| **kind == T::Dedent).count(), 6);
        assert_eq!(kinds.iter().filter(|kind| **kind == T::Newline).count(), 16);
        assert!(kinds.contains(&text("I am stuck")));
    }

    #[test]
    fn the_example_of_the_specifications_reads_with_crlf() {
        assert_eq!(tokens(&EXAMPLE.replace('\n', "\r\n")), tokens(EXAMPLE));
    }

    #[test]
    fn a_function_with_its_parameters() {
        assert_eq!(
            kinds("function paint_side(length, color):\n    return length\n"),
            vec![
                T::Keyword(K::Function),
                name("paint_side"),
                T::LeftParen,
                name("length"),
                T::Comma,
                name("color"),
                T::RightParen,
                T::Colon,
                T::Newline,
                T::Indent,
                T::Keyword(K::Return),
                name("length"),
                T::Newline,
                T::Dedent,
                T::End
            ]
        );
    }

    #[test]
    fn a_condition_with_else_if() {
        assert_eq!(
            kinds("if a:\n    b()\nelse if c:\n    break\nelse:\n    d = true\n"),
            vec![
                T::Keyword(K::If),
                name("a"),
                T::Colon,
                T::Newline,
                T::Indent,
                name("b"),
                T::LeftParen,
                T::RightParen,
                T::Newline,
                T::Dedent,
                T::Keyword(K::Else),
                T::Keyword(K::If),
                name("c"),
                T::Colon,
                T::Newline,
                T::Indent,
                T::Keyword(K::Break),
                T::Newline,
                T::Dedent,
                T::Keyword(K::Else),
                T::Colon,
                T::Newline,
                T::Indent,
                name("d"),
                T::Equal,
                T::Keyword(K::True),
                T::Newline,
                T::Dedent,
                T::End
            ]
        );
    }

    #[test]
    fn a_comparison_and_logic() {
        assert_eq!(
            line_kinds("while not (x >= 1 and x != 5) or false:"),
            vec![
                T::Keyword(K::While),
                T::Keyword(K::Not),
                T::LeftParen,
                name("x"),
                T::GreaterEqual,
                T::Number(1),
                T::Keyword(K::And),
                name("x"),
                T::NotEqual,
                T::Number(5),
                T::RightParen,
                T::Keyword(K::Or),
                T::Keyword(K::False),
                T::Colon
            ]
        );
    }
}

/// Sources drawn at random from pieces that stress the lexer, to check that it never panics
/// and that what it gives always holds together.
mod robustness {
    use super::*;

    const PIECES: [&str; 40] = [
        " ",
        "    ",
        "\t",
        "\n",
        "\r\n",
        "\r",
        "#",
        "\"",
        "'",
        "\\",
        "\\n",
        "\\\"",
        "a",
        "x_2",
        "côté",
        "Let",
        "if",
        "else",
        "let",
        "0",
        "42",
        "9223372036854775808",
        "2nd",
        "=",
        "==",
        "!",
        "!=",
        "<",
        ">=",
        "&",
        "|",
        "(",
        ")",
        ":",
        ",",
        "-",
        "@",
        "🎉",
        "\u{a0}",
        ".",
    ];

    /// A small generator of numbers, the same at every run, without a dependency.
    struct Random(u64);

    impl Random {
        fn next(&mut self) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            usize::try_from(self.0 >> 33).unwrap()
        }
    }

    #[test]
    fn random_sources_never_panic_and_hold_together() {
        let mut random = Random(7);
        let mut read = 0;
        for _ in 0..20_000 {
            let length = random.next() % 30;
            let source: String = (0..length)
                .map(|_| PIECES[random.next() % PIECES.len()])
                .collect();
            let Ok(tokens) = tokenize(&source) else {
                continue;
            };
            read += 1;
            assert_eq!(tokens.last().map(|token| &token.kind), Some(&T::End));
            let indents = tokens.iter().filter(|t| t.kind == T::Indent).count();
            let dedents = tokens.iter().filter(|t| t.kind == T::Dedent).count();
            assert_eq!(indents, dedents, "{source:?}");
            for pair in tokens.windows(2) {
                assert!(pair[0].span.end <= pair[1].span.start, "{source:?}");
            }
        }
        // Enough sources without error for the checks above to mean something.
        assert!(read > 1_000, "{read}");
    }

    #[test]
    fn random_indented_programs_balance_their_blocks() {
        let mut random = Random(11);
        for _ in 0..5_000 {
            let mut source = String::new();
            let mut depth = 0_usize;
            for _ in 0..random.next() % 20 {
                depth = match random.next() % 3 {
                    0 => depth + 1,
                    1 => depth.saturating_sub(random.next() % 4),
                    _ => depth,
                };
                source.push_str(&" ".repeat(depth * 4));
                source.push_str("a:\n");
            }
            let Ok(tokens) = tokenize(&source) else {
                // Only a line indented by more than one level at once is refused.
                assert!(matches!(error_kind(&source), E::WrongIndent { .. }));
                continue;
            };
            let indents = tokens.iter().filter(|t| t.kind == T::Indent).count();
            let dedents = tokens.iter().filter(|t| t.kind == T::Dedent).count();
            assert_eq!(indents, dedents, "{source:?}");
        }
    }
}
