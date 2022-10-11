use quick_xml::{Writer, Error};

pub struct HeaderInfo {
    pub chars: Vec<u32>,
    pub height: u32,
    pub name: String
}

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

fn write_font_properties(writer: &mut Writer<Vec<u8>>, data: &HeaderInfo) -> Result<(), Error> {
    writer
        .create_element("FontProperties")
        .with_attributes(vec![
            ("Baseline", "13"),
            ("Filter", ""),
            ("Height", data.height.to_string().as_str()),
            ("Name", data.name.as_str()),
            ("Space", "5"),
            ("Style", "p"),
            ("Width", "-1")
        ])
        .write_inner_content(|writer| {
            writer.create_element("Identifier")
                .with_attribute(("Value", "34"))
                .write_empty()?;
            Ok(())
        })?;
    Ok(())
}

fn write_character_propertiers(writer: &mut Writer<Vec<u8>>, data: &HeaderInfo) -> Result<(), Error> {
    writer
        .create_element("FontCharacterProperties")
        .write_inner_content(|writer| {
            for ch in data.chars.iter() {
                let index = format!("0x{:x}", ch); 
                writer.create_element("Character")
                    .with_attributes(vec![
                        ("Index", index.as_str()),
                        ("LeftSpace", "0"),
                        ("RightSpace", "0")
                    ])
                    .write_empty()?;
            }

            Ok(())
        })?;

    Ok(())
}

pub fn write_header(data: HeaderInfo) -> Result<Vec<u8>, Error> {
    let mut writer = Writer::new(Vec::new());
    writer
        .create_element("FontGenerator")
        .write_inner_content(|writer| {
            write_informations(writer)?;    // <Informations>
            write_font_properties(writer, &data)?; // <FontProperties>
            write_character_propertiers(writer, &data)?; // <FontCharacterProperties>
            Ok(())
        })?;
    Ok(writer.inner().to_vec())
}