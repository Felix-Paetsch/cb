use logos::{Lexer, Logos}; // ,Source
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // Regex um einen a-tag mit href von <a ... bis </a ...> zu finden (Lexer::Hohe Priorität)
    #[regex("<a[ \t\n\r][^>]*href=\"([^\"]*)\"[^>]*>([^<]*)</a[ \t\n\r]*>", extract_link_info, priority=200)]
    Link((LinkUrl, LinkText)),

    // Alternativ, wenn kein passender a tag gefunden wurde: geh weiter
    #[regex(".|\n|\r|\t", test_debug, priority=10)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    // Hauptproblem: Das Regex für den Link scheint etwas zu finden, aber diese Funktion wird nie aufgerufen
    // Auf der anderen Seite wird die test_debug Funktion auch nicht aufgerufen und es gibt auch keinen Fehler.

    // Annahme, da wir nicht testen können: lex.slice() -> das gesammte gematchte Regex
    lex.next();

    // Annahme: lex.slice() -> erste Klammer
    let url = LinkUrl(lex.slice().to_string());

    lex.next();
    // Annahme: lex.slice() -> zweite Klammer
    let text = LinkText(lex.slice().to_string());

    return (url, text)
}

fn test_debug(lex: &mut Lexer<URLToken>) {
    // Prüfen, ob dieser Token gefunden wird.

    // print!("{}", lex.slice().to_string());
    lex.next();
}
