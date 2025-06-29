use super::MachineW;

impl MachineW {
    pub fn set_input_buffer(&mut self, inputs: Vec<u16>) {
        self.input_buffer = inputs;
        self.input_buffer.reverse();
    }

    pub fn get_output_buffer(&self) -> &[u16] {
        &self.output_buffer
    }

    pub fn clear_output_buffer(&mut self) {
        self.output_buffer.clear();
    }
}
