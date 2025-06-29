#[derive(Debug, Clone, PartialEq)]
pub struct LabelDefinition {
    pub name: String,
    pub line: usize,
    pub column: usize,
}
