// Example: 0x0, 0x40-0x50,0x60-0x80

pub struct ParseError {
    pub input: String,
    pub message: String
}

pub fn parse_single_charcode(char_code: &str) -> Result<u32, ParseError> {
    // Trim spaces and ensure it starts with 0x.
    let char_code = char_code.trim();
    if !char_code.starts_with("0x") {
        return Err(ParseError {
            input: char_code.to_string(),
            message: format!("Number {} doesn't start with 0x", char_code)
        });
    }

    // Trim the 0x for parsing.
    let char_code = char_code.trim_start_matches("0x");

    let result = u32::from_str_radix(char_code, 16);
    if result.is_err() {
        return Err(ParseError {
            input: char_code.to_string(),
            message: format!("Number {} could not be parsed as a hexadecimal number.", char_code)
        });
    }

    Ok(result.unwrap())
}

pub fn char_range(descriptor: &String) -> Result<Vec<u32>, ParseError> {
    let descriptor = descriptor.replace(';', ",");
    let mut result = Vec::<u32>::new();
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