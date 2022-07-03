#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenKind {
    EMPTY,
    STRING(String),
    INTEGER(u64),
    IDENTIFIER,

    SepLt,     // <
    SepGt,     // >
    SepEq,     // =
    SepSemi,   // ;
    SepComma,  // ,
    SepLCurly, // {
    SepRCurly, // }
    SepLParen, // (
    SepRParen, // )

    KwAs,       // as
    KwMap,      // MAP
    KwEnum,     // enum
    KwVoid,     // void
    KwArray,    // Array
    KwImport,   // import
    KwService,  // service
    KwMessage,  // message
    KwOptional, // optional
    KwRequired, // required
}

#[derive(Debug)]
pub struct Token {
    pub line: u32,
    pub column: u32,
    pub content: String,
    pub kind: TokenKind,
}
