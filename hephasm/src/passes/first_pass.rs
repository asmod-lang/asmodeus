//! first pass: macro expansion

use crate::error::AssemblerError;
use crate::macro_processor::MacroProcessor;
use parseid::ast::{Program, ProgramElement};

pub struct FirstPass;

impl FirstPass {
    pub fn execute(
        macro_processor: &mut MacroProcessor,
        program: &Program
    ) -> Result<Vec<ProgramElement>, AssemblerError> {
        macro_processor.expand_macros(program)
    }
}
