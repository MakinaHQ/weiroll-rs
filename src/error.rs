use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum WeirollError {
    #[error("unable to plan")]
    PlanError,

    #[error("invalid return type")]
    InvalidReturnType,
    #[error("Call with value must have a value parameter")]
    MissingValue,

    // todo: panic on these?
    #[error("internal error: missing return slot")]
    MissingReturnSlot,

    #[error("internal error: invalid reserved slot type")]
    InvalidReservedSlot,

    #[error("internal error: invalid return slot")]
    InvalidReturnSlot,

    #[error("internal error: missing literal value")]
    MissingLiteralValue,

    #[error("argument count mismatch")]
    ArgumentCountMismatch,

    #[error("integer overflow")]
    InternalOverflow(#[from] std::num::TryFromIntError),

    #[error("command not visible here")]
    CommandNotVisible,

    #[error("a command takes at most 32 argument slots, got {0}")]
    TooManyArguments(usize),

    #[error("a plan uses at most 128 state slots, got {0}")]
    TooManyStateSlots(usize),
}
