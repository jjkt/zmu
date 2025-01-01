//!
//! Cortex Floating Point extension register operations
//!

use crate::core::bits::Bits;

/// Trait for accessing Floating Point registers
pub trait Fpscr {
    ///
    /// Get "N"egative flag value
    ///
    fn get_n(&self) -> bool;

    ///
    /// Set "N"egative flag value
    ///
    fn set_n(&mut self, n: bool);

    ///
    /// Get "Z"ero flag value
    ///
    fn get_z(&self) -> bool;
    ///
    /// Set "Z"ero flag value
    ///
    fn set_z(&mut self, z: bool);

    ///
    /// Get "C"arry flag value
    ///
    fn get_c(&self) -> bool;
    ///
    /// Set "C"arry flag value
    ///
    fn set_c(&mut self, c: bool);

    ///
    /// Get Overflow flag value
    ///
    fn get_v(&self) -> bool;
    ///
    /// Set Overflow flag value
    ///
    fn set_v(&mut self, v: bool);
}

impl Fpscr for u32 {
    fn get_n(&self) -> bool {
        self.get_bit(31)
    }

    fn set_n(&mut self, n: bool) {
        self.set_bit(31, n);
    }

    fn get_z(&self) -> bool {
        self.get_bit(30)
    }

    fn set_z(&mut self, z: bool) {
        self.set_bit(30, z)
    }

    fn get_c(&self) -> bool {
        self.get_bit(29)
    }

    fn set_c(&mut self, c: bool) {
        self.set_bit(29, c);
    }

    fn get_v(&self) -> bool {
        self.get_bit(28)
    }

    fn set_v(&mut self, v: bool) {
        self.set_bit(28, v);
    }
}
