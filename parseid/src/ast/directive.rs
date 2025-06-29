#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub name: String,
    pub arguments: Vec<String>,
    pub line: usize,
    pub column: usize,
}
