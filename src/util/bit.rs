pub mod bit {
    pub fn set_bit(byte: u8, bit_pos: u8, value: bool) -> u8 {
        if bit_pos < 8 {
            if value {
                byte | (1 << bit_pos)
            } else {
                byte & !(1 << bit_pos)
            }
        } else {
            panic!("Bit pos out of range (0-7)");
        }
    }

    pub fn get_bit(byte: u8, bit_pos: u8) -> bool {
        if bit_pos < 8 {
            byte & (1 << bit_pos) != 0
        } 
        else {
            panic!("Bit pos out of range (0-7)");
        }
    }
}