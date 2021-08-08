// src/error.rs
/// Zircon statuses are signed 32 bit integers. The space of values is
/// divided as follows:
/// - The zero value is for the OK status.
/// - Negative values are defined by the system, in this file.
/// - Positive values are reserved for protocol-specific error values,
///   and will never be defined by the system.
#[allow(non_camel_case_types, dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ZxError {
    OK = 0,

    // ======= Internal failures =======
    /// The system encountered an otherwise unspecified error
    /// while performing the operation.
    INTERNAL = -1,

    /// The operation is not implemented, supported,
    /// or enabled.
    NOT_SUPPORTED = -2,

    // ......

    /// Connection was aborted.
    CONNECTION_ABORTED = -76,
}

pub type ZxResult<T> = Result<T, ZxError>;