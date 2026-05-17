use super::*;

fn kinds(src: &str) -> Vec<TokenKind> {
    lex(src).expect("lex should succeed").into_iter().map(|t| t.kind).collect()
}

#[test]
fn empty_input_emits_eof() {
    assert_eq!(kinds(""), vec![TokenKind::Eof]);
}

#[test]
fn whitespace_only_is_eof() {
    assert_eq!(kinds("   \n\t\r\n"), vec![TokenKind::Eof]);
}

#[test]
fn keywords_and_idents() {
    assert_eq!(
        kinds("function add foo"),
        vec![
            TokenKind::Function,
            TokenKind::Ident("add".into()),
            TokenKind::Ident("foo".into()),
            TokenKind::Eof,
        ],
    );
}

#[test]
fn type_keywords() {
    assert_eq!(
        kinds("number string boolean"),
        vec![
            TokenKind::NumberKw,
            TokenKind::StringKw,
            TokenKind::BooleanKw,
            TokenKind::Eof,
        ],
    );
}

#[test]
fn number_literals() {
    assert_eq!(kinds("42"), vec![TokenKind::Number(42.0), TokenKind::Eof]);
    assert_eq!(kinds("3.14"), vec![TokenKind::Number(3.14), TokenKind::Eof]);
    // Trailing dot is NOT consumed into the number — Phase 1 keeps numeric
    // grammar tight to avoid clashing with `.` for member access.
    assert_eq!(
        kinds("1."),
        vec![TokenKind::Number(1.0), TokenKind::Dot, TokenKind::Eof],
    );
}

#[test]
fn string_literals_and_escapes() {
    assert_eq!(
        kinds(r#""hello""#),
        vec![TokenKind::Str("hello".into()), TokenKind::Eof],
    );
    assert_eq!(
        kinds(r#""a\nb""#),
        vec![TokenKind::Str("a\nb".into()), TokenKind::Eof],
    );
    assert_eq!(
        kinds("'single'"),
        vec![TokenKind::Str("single".into()), TokenKind::Eof],
    );
}

#[test]
fn comments_are_skipped() {
    assert_eq!(
        kinds("// a comment\nconst x;\n/* block */ let"),
        vec![
            TokenKind::Const,
            TokenKind::Ident("x".into()),
            TokenKind::Semi,
            TokenKind::Let,
            TokenKind::Eof,
        ],
    );
}

#[test]
fn multi_char_operators() {
    assert_eq!(
        kinds("== === != !=="),
        vec![
            TokenKind::EqEq,
            TokenKind::EqEqEq,
            TokenKind::NotEq,
            TokenKind::NotEqEq,
            TokenKind::Eof,
        ],
    );
    assert_eq!(
        kinds("<= >= < >"),
        vec![
            TokenKind::LtEq,
            TokenKind::GtEq,
            TokenKind::Lt,
            TokenKind::Gt,
            TokenKind::Eof,
        ],
    );
    assert_eq!(
        kinds("&& || !"),
        vec![TokenKind::AmpAmp, TokenKind::PipePipe, TokenKind::Bang, TokenKind::Eof],
    );
}

#[test]
fn spans_are_byte_offsets() {
    let toks = lex("const x").unwrap();
    assert_eq!(toks[0].span, Span { start: 0, end: 5 });
    assert_eq!(toks[1].span, Span { start: 6, end: 7 });
    assert_eq!(toks[2].kind, TokenKind::Eof);
}

#[test]
fn unterminated_string_is_error() {
    let err = lex(r#""nope"#).unwrap_err();
    assert!(matches!(err.kind, LexErrorKind::UnterminatedString));
}

#[test]
fn unexpected_char_is_error() {
    let err = lex("@").unwrap_err();
    assert!(matches!(err.kind, LexErrorKind::UnexpectedChar('@')));
}

#[test]
fn phase1_example_lexes() {
    let src = "\
function add(a: number, b: number): number {
  return a + b;
}

const x: number = add(2, 3);
console.log(x);
";
    let toks = lex(src).expect("phase 1 example should lex");
    // Spot-check a handful of expected tokens rather than enumerate all 36.
    assert!(matches!(toks[0].kind, TokenKind::Function));
    assert!(matches!(toks[1].kind, TokenKind::Ident(ref s) if s == "add"));
    assert!(matches!(toks.last().unwrap().kind, TokenKind::Eof));
}
