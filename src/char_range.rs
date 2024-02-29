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

pub fn parse_char(char_code: u32, skip_control_characters: bool) -> Option<char> {
    // Parse the character from u32.
    let ch = char::from_u32(char_code);
    if ch.is_none() {
        return None;
    }

    let ch = ch.unwrap();

    // Skip control characters, if needed.
    if skip_control_characters && ch.is_control() {
        return None;
    }

    // Skip spaces.
    if ch == ' ' {
        return None;
    }

    Some(ch)
}

pub fn char_range(descriptor: &String, skip_control_characters: bool, add_null_character: Option<bool>) -> Result<Vec<char>, ParseError> {
    let descriptor = descriptor.replace(';', ",");
    let mut result = Vec::<char>::new();

    if add_null_character.unwrap_or_default() {
        let ch = parse_char(0x00, false);
        if ch.is_some() {
            result.push(ch.unwrap());
        }
    }

    for item in descriptor.split(',') {
        let range = item.split_once('-');

        // The current item is a single character code (e.g. 0x60), not a range.
        if range.is_none() {
            let char_code = parse_single_charcode(item.trim())?;
            let ch = parse_char(char_code, skip_control_characters);

            if ch.is_some() {
                result.push(ch.unwrap());
            }
            continue;
        }

        // The current item is a range (e.g. 0x40-0x50)
        let range_segments = range.unwrap();
        let start = parse_single_charcode(range_segments.0)?;
        let end = parse_single_charcode(range_segments.1)?;

        for char_code in start..end {
            let ch = parse_char(char_code, skip_control_characters);
            if ch.is_some() {
                result.push(ch.unwrap());
            }
        }
    }
    Ok(result)
}