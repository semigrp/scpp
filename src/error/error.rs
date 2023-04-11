mod array_error;
mod memory_error;
mod pointer_error;

pub use array_error::ArrayError;
pub use memory_error::MemoryError;
pub use pointer_error::PointerError;

pub enum Error {
    Array(ArrayError),
    Memory(MemoryError),
    Pointer(PointerError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Array(array_error) => write!(f, "{}", array_error),
            Error::Memory(memory_error) => write!(f, "{}", memory_error),
            Error::Pointer(pointer_error) => write!(f, "{}", pointer_error),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Array(array_error) => write!(f, "{:?}", array_error),
            Error::Memory(memory_error) => write!(f, "{:?}", memory_error),
            Error::Pointer(pointer_error) => write!(f, "{:?}", pointer_error),
        }
    }
}

impl From<ArrayError> for Error {
    fn from(error: ArrayError) -> Self {
        Error::Array(error)
    }
}

impl From<MemoryError> for Error {
    fn from(error: MemoryError) -> Self {
        Error::Memory(error)
    }
}

impl From<PointerError> for Error {
    fn from(error: PointerError) -> Self {
        Error::Pointer(error)
    }
}
