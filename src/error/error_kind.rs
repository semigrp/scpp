pub enum ErrorKind {
    ArrayErrorKind,
    MemoryErrorKind,
    PointerErrorKind,
}

impl ErrorKind {
    // Returns a description of the error kind
    pub fn description(&self) -> &str {
        match self {
            ErrorKind::ArrayErrorKind => "Array error",
            ErrorKind::MemoryErrorKind => "Memory error",
            ErrorKind::PointerErrorKind => "Pointer error",
        }
    }

    // Converts a string to an ErrorKind, returning an error message if the string is not a valid error kind
    pub fn from_str(s: &str) -> Result<Self, &str> {
        match s {
            "ArrayErrorKind" => Ok(ErrorKind::ArrayErrorKind),
            "MemoryErrorKind" => Ok(ErrorKind::MemoryErrorKind),
            "PointerErrorKind" => Ok(ErrorKind::PointerErrorKind),
            _ => Err("Invalid error kind"),
        }
    }
}
