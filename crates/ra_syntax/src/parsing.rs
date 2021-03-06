//! Lexing, bridging to ra_parser (which does the actual parsing) and
//! incremental reparsing.

mod lexer;
mod text_token_source;
mod text_tree_sink;
mod reparsing;

use crate::{syntax_node::GreenNode, SyntaxError};
use text_token_source::TextTokenSource;
use text_tree_sink::TextTreeSink;

pub use lexer::*;

pub(crate) use self::reparsing::incremental_reparse;

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (tokens, lexer_errors) = tokenize(&text);

    let mut token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);

    ra_parser::parse(&mut token_source, &mut tree_sink);

    let (tree, mut parser_errors) = tree_sink.finish();
    parser_errors.extend(lexer_errors);

    (tree, parser_errors)
}
