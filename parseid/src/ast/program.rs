use super::{Instruction, LabelDefinition, Directive, MacroDefinition, MacroCall};

/// main program node containing all top-level elements
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub elements: Vec<ProgramElement>,
}

/// top-level program elements
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramElement {
    Instruction(Instruction),
    LabelDefinition(LabelDefinition),
    Directive(Directive),
    MacroDefinition(MacroDefinition),
    MacroCall(MacroCall),
}

impl Program {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn add_element(&mut self, element: ProgramElement) {
        self.elements.push(element);
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
