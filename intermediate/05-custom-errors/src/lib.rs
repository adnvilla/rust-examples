use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug)]
pub enum ReservationError {
    InvalidFormat,
    InvalidPartySize(ParseIntError),
    EmptyName,
    TooLarge { requested: u16, capacity: u16 },
}

impl fmt::Display for ReservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(formatter, "usa el formato nombre,cantidad"),
            Self::InvalidPartySize(_) => write!(formatter, "la cantidad no es un número válido"),
            Self::EmptyName => write!(formatter, "el nombre está vacío"),
            Self::TooLarge {
                requested,
                capacity,
            } => {
                write!(
                    formatter,
                    "se pidieron {requested} lugares, capacidad: {capacity}"
                )
            }
        }
    }
}

impl Error for ReservationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPartySize(error) => Some(error),
            _ => None,
        }
    }
}

/// Interpreta una reservación `nombre,cantidad` limitada por `capacity`.
///
/// # Errors
///
/// Devuelve [`ReservationError`] para formatos, nombres, cantidades o tamaños
/// inválidos.
pub fn parse_reservation(input: &str, capacity: u16) -> Result<(&str, u16), ReservationError> {
    let (name, size) = input
        .split_once(',')
        .ok_or(ReservationError::InvalidFormat)?;
    let name = name.trim();
    if name.is_empty() {
        return Err(ReservationError::EmptyName);
    }
    let size = size
        .trim()
        .parse()
        .map_err(ReservationError::InvalidPartySize)?;
    if size > capacity {
        return Err(ReservationError::TooLarge {
            requested: size,
            capacity,
        });
    }
    Ok((name, size))
}

#[cfg(test)]
mod tests {
    use super::{ReservationError, parse_reservation};

    #[test]
    fn preserves_context_for_large_groups() {
        let error = parse_reservation("Ferris,30", 8).expect_err("must exceed capacity");
        assert!(matches!(
            error,
            ReservationError::TooLarge {
                requested: 30,
                capacity: 8
            }
        ));
    }
}
