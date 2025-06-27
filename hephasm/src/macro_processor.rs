//! macro processing and expansion

use crate::error::AssemblerError;
use crate::types::ExpandedMacro;
use parseid::ast::*;
use std::collections::HashMap;

pub struct MacroProcessor {
    macros: HashMap<String, ExpandedMacro>,
}

impl MacroProcessor {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.macros.clear();
    }

    pub fn expand_macros(&mut self, program: &Program) -> Result<Vec<ProgramElement>, AssemblerError> {
        let mut expanded = Vec::new();

        for element in &program.elements {
            match element {
                ProgramElement::MacroDefinition(macro_def) => {
                    // store macro definition
                    self.macros.insert(
                        macro_def.name.clone(),
                        ExpandedMacro {
                            parameters: macro_def.parameters.clone(),
                            body: macro_def.body.clone(),
                        },
                    );
                }
                ProgramElement::MacroCall(macro_call) => {
                    let expanded_elements = self.expand_macro_call(macro_call)?;
                    expanded.extend(expanded_elements);
                }
                _ => {
                    // copy other elements as-is
                    expanded.push(element.clone());
                }
            }
        }

        Ok(expanded)
    }

    fn expand_macro_call(&self, macro_call: &MacroCall) -> Result<Vec<ProgramElement>, AssemblerError> {
        let macro_def = self.macros.get(&macro_call.name)
            .ok_or_else(|| AssemblerError::MacroNotFound {
                name: macro_call.name.clone(),
                line: macro_call.line,
            })?;

        if macro_def.parameters.len() != macro_call.arguments.len() {
            return Err(AssemblerError::MacroParameterMismatch {
                name: macro_call.name.clone(),
                expected: macro_def.parameters.len(),
                found: macro_call.arguments.len(),
                line: macro_call.line,
            });
        }

        // parameter substitution map
        let mut substitutions = HashMap::new();
        for (param, arg) in macro_def.parameters.iter().zip(macro_call.arguments.iter()) {
            substitutions.insert(param.clone(), arg.clone());
        }

        // substitute parameters in macro body
        let mut expanded = Vec::new();
        for element in &macro_def.body {
            expanded.push(self.substitute_parameters(element, &substitutions));
        }

        Ok(expanded)
    }

    fn substitute_parameters(&self, element: &ProgramElement, substitutions: &HashMap<String, String>) -> ProgramElement {
        match element {
            ProgramElement::Instruction(inst) => {
                let mut new_inst = inst.clone();
                if let Some(operand) = &mut new_inst.operand {
                    operand.value = self.substitute_in_string(&operand.value, substitutions);
                }
                ProgramElement::Instruction(new_inst)
            }
            ProgramElement::Directive(dir) => {
                let mut new_dir = dir.clone();
                for arg in &mut new_dir.arguments {
                    *arg = self.substitute_in_string(arg, substitutions);
                }
                ProgramElement::Directive(new_dir)
            }
            _ => element.clone(),
        }
    }

    fn substitute_in_string(&self, s: &str, substitutions: &HashMap<String, String>) -> String {
        // entire string matches a parameter = replace it
        if let Some(value) = substitutions.get(s) {
            return value.clone();
        }
        
        // otherwise return the original string
        s.to_string()
    }
}

impl Default for MacroProcessor {
    fn default() -> Self {
        Self::new()
    }
}
