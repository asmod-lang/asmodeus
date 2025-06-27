//! Unit tests for the dismael disassembler module

mod unit {
    pub mod basic_disassembly_tests;
    pub mod label_tests;
    pub mod data_recognition_tests;
    pub mod instruction_category_tests;
    pub mod advanced_disassembler_tests;
    pub mod error_tests;
    pub mod performance_tests;
}

pub use unit::*;
