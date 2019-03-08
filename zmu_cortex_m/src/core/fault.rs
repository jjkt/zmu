//!
//! A Trait for representing a Cortex-M fault
//!
//!

#[derive(PartialEq, Debug, Copy, Clone)]
///
/// Fault types
///
pub enum Fault {
    ///
    /// Bus error happened while reading the vector table entry
    ///
    VectorTable,
    ///
    ///
    ///
    Forced,
    ///
    ///
    ///
    IAccViol,
    ///
    ///
    ///
    DAccViol,
    ///
    ///
    ///
    Mstkerr,
    ///
    ///
    ///
    Msunskerr,
    ///
    ///
    ///
    Stkerr,
    ///
    /// Unknown instruction was tried to be executed.
    ///
    UndefInstr,
    ///
    Invstate,
    ///
    InvPc,
    ///
    Unaligned,
    ///
    ///
    DivByZero,
}
