use asmachina::MachineW;

#[test]
fn test_original_array_iteration_algorithm() {
    let mut machine = MachineW::new();
    
    // int count_lower_than_a(int a, int tab[], int n)
    let program = vec![
        // loop: POB n (address 0)
        (0b00100 << 11) | 15,  // POB 15 (n variable)
        // ODE one (address 1)
        (0b00010 << 11) | 17,  // ODE 17 (one variable)
        // SOM end (address 2)
        (0b00110 << 11) | 25,  // SOM 25 (end label)
        // ŁAD n (address 3)
        (0b00011 << 11) | 15,  // LAD 15 (store back to n)
        
        // calc: POB tab (address 4) - this instruction gets modified!
        (0b00100 << 11) | 11,  // POB 11 (initially tab[0])
        // ODE a (address 5)
        (0b00010 << 11) | 10,  // ODE 10 (threshold a)
        // SOM cpp (address 6)
        (0b00110 << 11) | 19,  // SOM 19 (cpp label)
        
        // return: POB calc (address 7) - loads address of calc instruction
        (0b00100 << 11) | 4,   // POB 4 (calc instruction address)
        // DOD one (address 8)
        (0b00001 << 11) | 17,  // DOD 17 (increment address by 1)
        // ŁAD calc (address 9) - modifies calc instruction!
        (0b00011 << 11) | 4,   // LAD 4 (store modified instruction)
        // SOB loop (address 10)
        (0b00101 << 11) | 0,   // SOB 0 (back to loop)
        
        // variables
        4,      // a: threshold value (address 10)
        1,      // tab[0] (address 11)
        3,      // tab[1] (address 12)
        5,      // tab[2] (address 13)
        4,      // tab[3] (address 14)
        7,      // tab[4] (address 15) - this conflicts with n!
        5,      // n: array length (address 16) - moved here
        1,      // one: constant 1 (address 17)
        0,      // count: counter (address 18)
        
        // cpp: POB count (address 19)
        (0b00100 << 11) | 18,  // POB 18 (count variable)
        // DOD one (address 20)
        (0b00001 << 11) | 17,  // DOD 17 (one variable)
        // ŁAD count (address 21)
        (0b00011 << 11) | 18,  // LAD 18 (store back to count)
        // SOB return (address 22)
        (0b00101 << 11) | 7,   // SOB 7 (back to return)
        0, 0,   // padding (addresses 23-24)
        
        // end: POB count (address 25)
        (0b00100 << 11) | 18,  // POB 18 (count variable)
        // WYJSCIE (address 26)
        (0b01111 << 11) | 0,   // WYJSCIE (output result)
        // STP (address 27)
        (0b00111 << 11) | 0,   // STP
    ];
    
    let mut corrected_program = program;
    corrected_program[0] = (0b00100 << 11) | 16;  // POB 16 (n at new address)
    corrected_program[3] = (0b00011 << 11) | 16;  // LAD 16 (n at new address)
    
    assert!(machine.load_program(&corrected_program).is_ok());
    
    // step limit to prevent infinite loops
    let result = machine.run_steps(1000);
    
    match result {
        Ok(steps) => {
            println!("Program executed {} steps", steps);
            println!("Final AK: {}", machine.ak);
            println!("Output buffer: {:?}", machine.get_output_buffer());
            println!("Is running: {}", machine.is_running);
            
            if !machine.is_running && !machine.get_output_buffer().is_empty() {
                assert_eq!(machine.get_output_buffer()[0], 2, "Should count 2 elements lower than 4");
            }
        }
        Err(e) => {
            println!("Program error: {:?}", e);
            println!("Machine state - AK: {}, L: {}", machine.ak, machine.l);
            println!("Memory around variables: {:?}", &machine.memory[10..20]);
        }
    }
}
