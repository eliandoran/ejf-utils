use quick_xml::{Writer, Error};

fn write_informations(writer: &mut Writer<Vec<u8>>) -> Result<(), Error> {
    writer
        .create_element("Informations")
        .with_attributes(vec![
            ("Vendor", "IS2T"),
            ("Version", "0.8")
        ])
        .write_empty()?;
    Ok(())
}

fn write_font_properties(writer: &mut Writer<Vec<u8>>, height: u32) -> Result<(), Error> {
    writer
        .create_element("FontProperties")
        .with_attribute(("Baseline", "13"))
        .with_attribute(("Filter", "u"))
        .with_attribute(("Height", height.to_string().as_str()))
        .with_attribute(("Name", "Foo"))
        .with_attribute(("Space", "5"))
        .with_attribute(("Style", "pu"))
        .with_attribute(("Width", "-1"))
        .write_inner_content(|writer| {
            writer.create_element("Identifier")
                .with_attribute(("Value", "34"))
                .write_empty()?;
            Ok(())
        })?;
    Ok(())
}

fn write_character_propertiers(writer: &mut Writer<Vec<u8>>, chars: &[char]) -> Result<(), Error> {
    writer
        .create_element("FontCharacterProperties")
        .write_inner_content(|writer| {
            for ch in chars.iter() {
                let index = format!("0x{:x}", (*ch) as u8); 
                writer.create_element("Character")
                    .with_attribute(("Index", index.as_str()))
                    .with_attribute(("LeftSpace", "0"))
                    .with_attribute(("RightSpace", "0"))
                    .write_empty()?;
            }

            Ok(())
        })?;

    Ok(())
}

pub fn write_header(chars: &[char], height: u32) -> Result<Vec<u8>, Error> {
    let mut writer = Writer::new(Vec::new());
    writer
        .create_element("FontGenerator")
        .write_inner_content(|writer| {
            write_informations(writer)?;    // <Informations>
            write_font_properties(writer, height)?; // <FontProperties>
            write_character_propertiers(writer, chars)?; // <FontCharacterProperties>
            Ok(())
        })?;
    Ok(writer.inner().to_vec())
}