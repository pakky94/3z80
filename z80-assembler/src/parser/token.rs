#[derive(Debug, Eq, PartialEq)]
pub enum Token<'a> {
    Label(&'a str),
    EOF
}
