#[derive(Debug, PartialEq, Eq)]
pub struct LogEntry<'a> {
    pub level: &'a str,
    pub message: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingSeparator,
    EmptyLevel,
    EmptyMessage,
}

/// Interpreta `NIVEL|mensaje` devolviendo slices de la entrada.
///
/// # Errors
///
/// Devuelve [`ParseError`] si falta el separador o alguno de los campos.
pub fn parse_line(line: &str) -> Result<LogEntry<'_>, ParseError> {
    let (level, message) = line.split_once('|').ok_or(ParseError::MissingSeparator)?;
    if level.is_empty() {
        return Err(ParseError::EmptyLevel);
    }
    if message.is_empty() {
        return Err(ParseError::EmptyMessage);
    }
    Ok(LogEntry { level, message })
}

#[cfg(test)]
mod tests {
    use super::{LogEntry, parse_line};

    #[test]
    fn fields_borrow_from_the_input() {
        assert_eq!(
            parse_line("INFO|café listo"),
            Ok(LogEntry {
                level: "INFO",
                message: "café listo"
            })
        );
    }
}
