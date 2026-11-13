/// Quote one SQL identifier, including embedded double quotes.
/// This is not a value escaper or an InfluxQL/Flux quoting function.
pub fn quote_identifier(identifier: &str) -> String {
    format!("\"{}\"", identifier.replace('"', "\"\""))
}

#[cfg(test)]
mod tests;
