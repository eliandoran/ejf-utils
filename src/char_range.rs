// Example: 0x0, 0x40-0x50,0x60-0x80

pub struct ParseError {
    pub input: String,
    pub message: String
}

pub fn parse_single_charcode(char_code: &str) -> Result<u8, ParseError> {
    let trimmed = char_code.trim();
    if !trimmed.starts_with("0x") {
        return Err(ParseError {
            input: trimmed.to_string(),
            message: format!("Number {} doesn't start with 0x", trimmed)
        });
    }

    let result = u8::from_str_radix(trimmed.trim_start_matches("0x"), 16);
    if result.is_err() {
        return Err(ParseError {
            input: trimmed.to_string(),
            message: format!("Number {} could not be parsed as a hexadecimal number.", trimmed)
        });
    }

    Ok(result.unwrap())
}

pub fn char_range(descriptor: String) -> Result<Vec<u8>, ParseError> {
    let mut result = Vec::<u8>::new();
    for item in descriptor.split(',') {
        let range = item.split_once('-');

        // The current item is a single character code (e.g. 0x60), not a range.
        if range.is_none() {
            let ch = parse_single_charcode(item.trim());
            result.push(ch?);
            continue;
        }

        // The current item is a range (e.g. 0x40-0x50)
        let range_segments = range.unwrap();
        let start = parse_single_charcode(range_segments.0)?;
        let end = parse_single_charcode(range_segments.1)?;

        for ch in start..end {
            result.push(ch);
        }
    }
    Ok(result)
}