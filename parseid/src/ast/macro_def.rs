use super::ProgramElement;

#[derive(Debug, Clone, PartialEq)]
pub struct MacroDefinition {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<ProgramElement>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroCall {
    pub name: String,
    pub arguments: Vec<String>,
    pub line: usize,
    pub column: usize,
}
