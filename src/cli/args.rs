//! CLI argument structures and modes

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Assemble,
    Disassemble,
    Run,
    Debug,
    Interactive,
    Examples,
    New,
    Check,
    Format,
    Help,
}

#[derive(Debug, Clone)]
pub struct Args {
    pub mode: Mode,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub verbose: bool,
    pub debug: bool,
    pub extended: bool,
    pub watch: bool,
}
