#[derive(Debug, PartialEq, Eq)]
pub enum WindowError {
    OutOfBounds,
}

/// Devuelve una ventana prestada después de comprobar sus límites.
///
/// # Errors
///
/// Devuelve [`WindowError::OutOfBounds`] si el rango solicitado no cabe.
#[allow(unsafe_code)]
pub fn window(bytes: &[u8], start: usize, length: usize) -> Result<&[u8], WindowError> {
    let end = start.checked_add(length).ok_or(WindowError::OutOfBounds)?;
    if end > bytes.len() {
        return Err(WindowError::OutOfBounds);
    }

    // SAFETY: `start + length` fue comprobado sin overflow y no excede `bytes`.
    // El puntero deriva del slice original y el resultado conserva su lifetime.
    Ok(unsafe { std::slice::from_raw_parts(bytes.as_ptr().add(start), length) })
}

#[cfg(test)]
mod tests {
    use super::{WindowError, window};

    #[test]
    fn safe_api_guards_the_unsafe_operation() {
        assert_eq!(window(b"abcdef", 2, 3), Ok(&b"cde"[..]));
        assert_eq!(window(b"abc", 2, 9), Err(WindowError::OutOfBounds));
    }
}
