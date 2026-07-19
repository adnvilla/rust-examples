use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub product: String,
    pub quantity: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    MissingProduct,
    MissingQuantity,
    InvalidQuantity,
    UnexpectedArgument(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProduct => write!(formatter, "falta PRODUCTO"),
            Self::MissingQuantity => write!(formatter, "falta CANTIDAD"),
            Self::InvalidQuantity => write!(formatter, "CANTIDAD debe ser un entero positivo"),
            Self::UnexpectedArgument(value) => write!(formatter, "argumento inesperado: {value}"),
        }
    }
}

/// Convierte argumentos en una configuración tipada.
///
/// # Errors
///
/// Devuelve [`ConfigError`] cuando faltan argumentos o la cantidad es inválida.
pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Config, ConfigError> {
    let mut args = args.into_iter();
    let product = args.next().ok_or(ConfigError::MissingProduct)?;
    let quantity_text = args.next().ok_or(ConfigError::MissingQuantity)?;
    let quantity = quantity_text
        .parse()
        .map_err(|_| ConfigError::InvalidQuantity)?;
    if quantity == 0 {
        return Err(ConfigError::InvalidQuantity);
    }
    if let Some(extra) = args.next() {
        return Err(ConfigError::UnexpectedArgument(extra));
    }
    Ok(Config { product, quantity })
}

#[cfg(test)]
mod tests {
    use super::{Config, parse_args};

    #[test]
    fn parses_two_arguments() {
        assert_eq!(
            parse_args(["café".to_owned(), "3".to_owned()]),
            Ok(Config {
                product: "café".to_owned(),
                quantity: 3
            })
        );
    }
}
