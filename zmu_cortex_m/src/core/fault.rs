//!
//! A Trait for representing a Cortex-M fault
//!
//!

#[derive(PartialEq, Debug, Copy, Clone)]
///
/// Fault types
/// See Armv7-M Architecture Reference Manual, DDI0403E.e, B1.5.14 List of Armv7-M faults
///
pub enum Fault {
    ///
    /// Bus error on vector table read
    ///
    VectorTable,
    ///
    /// Fault or supervisor call escalation to hard fault.
    ///
    Forced,
    ///
    /// Hardfault on BKPT escalation
    ///
    DebugEvt,
    ///
    /// Bus fault on exception entry stack operations
    ///
    Stkerr,
    ///
    /// Memmanage fault on exception entry with stack operations
    ///
    Mstkerr,
    ///
    /// Bus fault on exception return stack operations
    ///
    Msunskerr,
    ///
    /// Memmanage fault on data access
    ///
    DAccViol,
    ///
    /// Memmanage fault on instruction access
    ///
    IAccViol,
    ///
    /// Busfault on instruction fetch, precise access
    ///
    IBusErr,
    ///
    /// Busfault on data access, precise
    ///
    Preciserr,
    ///
    /// Busfault on data access, imprecise
    ///
    Impreciseerr,
    ///
    /// Usage fault, no coprocessor
    ///
    Nocp,
    ///
    /// Unknown instruction was tried to be executed.
    ///
    UndefInstr,
    ///
    /// Usage fault attempt to execute an instruction that is not permitted in the current state.
    Invstate,
    ///
    /// Usage fault on exception return integrity check
    ///
    InvPc,
    ///
    /// Usage fault illegal unaligned memory load or store
    ///
    Unaligned,
    ///
    /// Divide number by zero
    ///
    DivByZero,
    ///
    /// Memmanage fault, delayed fp processing
    ///
    MlspErr,
    ///
    /// Busfault, delayed fp preservation
    ///
    LspErr,
}
