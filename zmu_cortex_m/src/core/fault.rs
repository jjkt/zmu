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
    VectorReadError,
    ///
    /// SVCall while handler group priority is <= excecution group priority
    ///
    FaultEscalation,
    ///
    /// Bus error while saving context in hardware
    ///
    MemoryFaultOnStackEntry,
    ///
    /// Bus error while restoring context in hardware
    ///
    MemoryFaultOnStackReturn,
    ///
    /// Bus error on instruction fetch or attempt to execute from XN memory
    ///
    MemoryFaultOnInstructionAccess,
    ///
    /// Precise error on explicit memory access
    ///
    PreciseDataAccess,
    ///
    /// Imprecise error on explicit memory access
    ///
    ImpreciseDataAccess,
    ///
    /// Unknown instruction was tried to be executed.
    ///
    UndefinedInstruction,
    ///
    /// Attempt to execute instruction when ESPR.T == 0
    ///
    InvalidThumbState,
    ///
    /// Any load-store instruction tried to access non-aligned location
    ///
    UnalignedLoadStore,
    ///
    /// Permission fault, mem access not matching all access conditions of region address match.
    ///
    MPUIllegalMemoryAccess,
    ///
    /// Attempt to execute illegal instruction from XN memory
    ///
    MPUIllegalInstructionXN,
    ///
    /// PPB access is not permitted.
    ///
    PPBUnprivilegedAccess,

    ///
    /// 
    DivideByZero,
}
