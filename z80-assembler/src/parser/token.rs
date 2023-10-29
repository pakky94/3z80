#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Label(&'a str),
    Identifier(&'a str),
    NewLine,
    EOF
}
