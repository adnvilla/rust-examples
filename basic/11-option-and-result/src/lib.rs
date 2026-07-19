use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum OrderError {
    MissingSeparator,
    InvalidQuantity,
    EmptyProduct,
}

impl fmt::Display for OrderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSeparator => write!(formatter, "falta ':' entre cantidad y producto"),
            Self::InvalidQuantity => write!(formatter, "la cantidad no es un entero positivo"),
            Self::EmptyProduct => write!(formatter, "el producto está vacío"),
        }
    }
}

/// Interpreta un pedido con el formato `cantidad: producto`.
///
/// # Errors
///
/// Devuelve un [`OrderError`] cuando falta el separador, la cantidad no es un
/// entero positivo o el nombre del producto está vacío.
pub fn parse_order(input: &str) -> Result<(u16, &str), OrderError> {
    let (quantity, product) = input.split_once(':').ok_or(OrderError::MissingSeparator)?;
    let quantity = quantity
        .trim()
        .parse::<u16>()
        .map_err(|_| OrderError::InvalidQuantity)?;
    let product = product.trim();

    if quantity == 0 {
        return Err(OrderError::InvalidQuantity);
    }
    if product.is_empty() {
        return Err(OrderError::EmptyProduct);
    }

    Ok((quantity, product))
}

#[cfg(test)]
mod tests {
    use super::{OrderError, parse_order};

    #[test]
    fn parses_a_valid_order() {
        assert_eq!(parse_order("2: café"), Ok((2, "café")));
    }

    #[test]
    fn explains_a_missing_separator() {
        assert_eq!(parse_order("dos cafés"), Err(OrderError::MissingSeparator));
    }
}
