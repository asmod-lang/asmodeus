use asmachina::MachineW;
use crate::error::AsmodeusError;
use crate::cli::Args;

pub fn run_interactive_program(machine_code: &[u16], args: &Args) -> Result<(), AsmodeusError> {
    println!("🔤 Asmodeus Interactive Mode");
    println!("Character-based I/O enabled - type characters for real-time processing");
    println!("Press Ctrl+C to interrupt\n");
    
    let mut machine = MachineW::new();
    machine.set_interactive_mode(true);
    machine.load_program(machine_code)?;
    
    if args.verbose {
        println!("Program loaded: {} words", machine_code.len());
        println!("Interactive character I/O mode: ON");
        println!("Program starting...\n");
    }
    
    match machine.run() {
        Ok(_) => {
            println!("\n✅ Program completed successfully.");
            if args.verbose {
                println!("Final machine state:");
                println!("AK: {:04X} ({})", machine.ak, machine.ak);
                if !machine.get_output_buffer().is_empty() {
                    println!("Output buffer: {:?}", machine.get_output_buffer());
                }
            }
        }
        Err(e) => {
            eprintln!("\n❌ Execution error: {}", e);
            return Err(AsmodeusError::MachineError(e));
        }
    }
    
    Ok(())
}
