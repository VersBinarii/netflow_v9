#[macro_use]
extern crate nom;

mod formaters;
mod templates;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
struct NetflowHeader {
    version: u16,
    count: u16,
    sys_uptime: u32,
    timestamp: u32,
    sequence: u32,
    source_id: u32,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct TypeLenHeader {
    flowset_id: u16,
    length: u16,
}

#[derive(Debug, Clone)]
struct TemplateHeader {
    template_id: u16,
    field_count: u16,
}

#[derive(Debug, Clone)]
struct TemplateField {
    field: u16,
    len: u16,
}

#[derive(Debug, Clone)]
struct TemplateFlowset {
    tl_header: TypeLenHeader,
    template_header: TemplateHeader,
    payload: Vec<TemplateField>,
}

#[derive(Debug, Clone)]
struct OptionTemplateHeader {
    template_id: u16,
    scope_len: u16,
    option_len: u16,
}

#[derive(Debug, Clone)]
struct OptionTemplate {
    tl_header: TypeLenHeader,
    options_template_header: OptionTemplateHeader,
    payload: Vec<TemplateField>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DataFlowset<'a> {
    source_ip: Option<&'a std::net::IpAddr>,
    #[serde(rename = "header")]
    tl_header: TypeLenHeader,
    #[serde(with = "resolve_hashmap")]
    records: HashMap<u16, &'a [u8]>,
}

mod resolve_hashmap {
    use crate::templates::TemplateFieldType;
    use serde::ser::{self, SerializeMap};
    use std::collections::HashMap;

    pub fn serialize<'a, S>(
        hash_map: &HashMap<u16, &'a [u8]>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut map = serializer.serialize_map(Some(hash_map.len()))?;
        for (k, v) in hash_map {
            let record = TemplateFieldType::from(*k);
            let parser = record.get_parser();
            let p = parser(v);
            map.serialize_entry(&record.to_string(), &p)?;
        }
        map.end()
    }
}

impl<'a> DataFlowset<'a> {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn set_source_ip(&mut self, addr: &'a std::net::IpAddr) {
        self.source_ip = Some(addr)
    }
}

#[derive(Clone)]
pub struct Parser {
    template_cache: HashMap<u16, TemplateFlowset>,
    options_cache: HashMap<u16, OptionTemplate>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            template_cache: HashMap::new(),
            options_cache: HashMap::new(),
        }
    }

    pub fn parse_netflow_packet<'a, 'b>(
        &'a mut self,
        packet: &'b [u8],
        addr: &'b std::net::IpAddr,
    ) -> Result<Vec<DataFlowset<'b>>, &'static str> {
        //20 bytes Netflow packet header
        let mut data = packet;

        if let Ok((buffer, header)) = parse_netflow_header(data) {
            match header.version {
                9 => {}
                _ => return Err("Unrecognized version"),
            }

            data = buffer;

            let mut flowset_count = header.count;
            let mut data_flowsets = Vec::<DataFlowset>::new();
            loop {
                if let Ok((buffer, tl_header)) = parse_tl_header(data) {
                    data = buffer;

                    match tl_header.flowset_id {
                        // We have a template
                        0 => {
                            if let Ok((buffer, template_flowset)) =
                                parse_template(data, tl_header)
                            {
                                data = buffer;
                                self.template_cache.insert(
                                    template_flowset
                                        .template_header
                                        .template_id,
                                    template_flowset,
                                );
                            } else {
                                return Err("Failed to parse the template");
                            }
                        }

                        // Option template
                        1 => {
                            if let Ok((buffer, template_flowset)) =
                                parse_options_template(data, tl_header)
                            {
                                data = buffer;
                                self.options_cache.insert(
                                    template_flowset
                                        .options_template_header
                                        .template_id,
                                    template_flowset,
                                );
                            } else {
                                return Err(
                                    "Failed to parse the options template",
                                );
                            }
                        }

                        // A dataset
                        _ => {
                            if tl_header.flowset_id < 255 {
                                return Err("Flowset ID out of range");
                            }
                            // Get the template fromthe cache
                            if self
                                .template_cache
                                .contains_key(&tl_header.flowset_id)
                            {
                                let template = {
                                    self.template_cache
                                        .get(&tl_header.flowset_id)
                                        .unwrap()
                                };
                                if let Ok((buffer, flowsets)) =
                                    parse_dataset(data, tl_header, &template)
                                {
                                    data = buffer;
                                    for mut f in flowsets {
                                        f.set_source_ip(addr);
                                        data_flowsets.push(f);
                                    }
                                } else {
                                    return Err("Failed to parse the dataset");
                                }
                            } else if self
                                .options_cache
                                .contains_key(&tl_header.flowset_id)
                            {
                                data = &data[(tl_header.length - 4) as usize..];
                            } else {
                                data = &data[(tl_header.length - 4) as usize..];
                            }
                        }
                    }
                }
                // we failed to parse it so just try another one
                flowset_count -= 1;

                if flowset_count == 0 {
                    return Ok(data_flowsets);
                }
            }
        }
        Err("Failed to parse header")
    }
}

named!(parse_netflow_header<&[u8], NetflowHeader>, do_parse!(
    version_and_count: bits!(tuple!(take_bits!(16u16), take_bits!(16u16))) >>
        uptime: bits!(take_bits!(32u32)) >>
        timestamp: bits!(take_bits!(32u32)) >>
        seq: bits!(take_bits!(32u32)) >>
        source_id: bits!(take_bits!(32u32)) >>
        (NetflowHeader {
            version: version_and_count.0,
            count: version_and_count.1,
            sys_uptime: uptime,
            timestamp: timestamp,
            sequence: seq,
            source_id: source_id
        })
));

named!(parse_tl_header<&[u8], TypeLenHeader>, do_parse!(
    flowset_id: bits!(take_bits!(16u16)) >>
        length: bits!(take_bits!(16u16)) >>
        (TypeLenHeader {
            flowset_id: flowset_id,
            length: length
        })
));

named!(parse_template_header<&[u8], TemplateHeader>, do_parse!(
    template_id: bits!(take_bits!(16u16)) >>
       field_count: bits!(take_bits!(16u16)) >>
        (TemplateHeader {
            template_id: template_id,
            field_count: field_count
        })
));

named!(parse_template_fields<&[u8], TemplateField>, do_parse!(
    field: bits!(take_bits!(16u16)) >>
        len: bits!(take_bits!(16u16)) >>
        (TemplateField {
            field: field,
            len: len
        })
));

named!(parse_option_template_header<&[u8], OptionTemplateHeader>, do_parse!(
    template_id: bits!(take_bits!(16u16)) >>
        scope_len: bits!(take_bits!(16u16)) >>
        option_len: bits!(take_bits!(16u16)) >>
        (OptionTemplateHeader {
            template_id: template_id,
            scope_len: scope_len,
            option_len: option_len
        })
));

fn parse_template<'a>(
    mut buffer: &'a [u8],
    tl_header: TypeLenHeader,
) -> Result<(&'a [u8], TemplateFlowset), ()> {
    // Adjust for TypeLenHeader size already parsed
    let mut byte_count = tl_header.length as usize - 4;
    // Keep parsed templates fields locally
    let mut template_fields: Vec<TemplateField> = Vec::new();

    if let Ok((bytes, template_header)) = parse_template_header(buffer) {
        // Keep a track of bytes to parse but the
        // "field_count" should be enough to parse it correctly
        byte_count -= buffer.len() - bytes.len();
        // Ensure the correct buffer is parsed
        buffer = bytes;
        let field_count = template_header.field_count;

        for _ in 0..field_count {
            if let Ok((bytes, template_field)) = parse_template_fields(buffer) {
                byte_count -= buffer.len() - bytes.len();
                buffer = bytes;
                template_fields.push(template_field);
            } else {
                return Err(());
            }
        }
        if byte_count == 0 {
            // Looks like we parsed all fields so add the template if we dont have it already
            return Ok((
                buffer,
                TemplateFlowset {
                    tl_header,
                    template_header,
                    payload: template_fields,
                },
            ));
        }
    }
    Err(())
}

fn parse_options_template<'a>(
    mut buffer: &'a [u8],
    tl_header: TypeLenHeader,
) -> Result<(&'a [u8], OptionTemplate), ()> {
    let mut template_fields: Vec<TemplateField> = Vec::new();
    let mut byte_count = 4; //Adjust for header length
    if let Ok((bytes, template_header)) = parse_option_template_header(buffer) {
        byte_count += buffer.len() - bytes.len();
        // Ensure the correct buffer is parsed
        buffer = bytes;

        // Parse first the scope fields
        let scope_len = template_header.scope_len / 4; // len is in bytes we're parsing 2 x u16
        for _ in 0..scope_len {
            if let Ok((bytes, template_field)) = parse_template_fields(buffer) {
                byte_count += buffer.len() - bytes.len();
                buffer = bytes;
                template_fields.push(template_field);
            } else {
                return Err(());
            }
        }

        // ...then the option fields
        let option_len = template_header.option_len / 4;
        for _ in 0..option_len {
            if let Ok((bytes, template_field)) = parse_template_fields(buffer) {
                byte_count += buffer.len() - bytes.len();
                buffer = bytes;
                template_fields.push(template_field);
            } else {
                return Err(());
            }
        }

        // Adjust for any remaining padding
        return Ok((
            &buffer[tl_header.length as usize - byte_count..],
            OptionTemplate {
                tl_header,
                options_template_header: template_header,
                payload: template_fields,
            },
        ));
    }
    Err(())
}

fn parse_dataset<'a>(
    buffer: &'a [u8],
    tl_header: TypeLenHeader,
    template: &TemplateFlowset,
) -> Result<(&'a [u8], Vec<DataFlowset<'a>>), ()> {
    let mut dataflows = Vec::new();
    let mut idx: usize = 0;
    let fields = &template.payload;
    let length = tl_header.length as usize - 4;
    while idx < length {
        let mut records = HashMap::new();
        for f in fields {
            let value = &buffer[idx..idx + f.len as usize];
            records.insert(f.field, value);
            idx += f.len as usize;
        }
        dataflows.push(DataFlowset {
            source_ip: None,
            tl_header,
            records,
        })
    }
    // Adjust for possible remaining padding
    Ok((&buffer[length..], dataflows))
}
