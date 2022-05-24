use axum::http::StatusCode;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
// use serde_json::Result as SerdeResult;
use std::collections::HashMap;

// @TODO discuss whether to use CamelCase and to_string() transformation.
// But if we use CSV crate, see also why CSV crate shortens "appt_suite_number"
// to "appt_suite_nu" -
// if that is not negotiable, we may want our own field mapping even more.
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
enum AddressType {
    appt,
    house,
    suite,
}

impl TryFrom<&str> for AddressType {
    type Error = StatusCode;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "appt" => Ok(Self::appt),
            "house" => Ok(Self::house),
            "suite" => Ok(Self::suite),
            _ => Err(StatusCode::NOT_ACCEPTABLE),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    reference: String,
    address_type: AddressType,
    suite_number: Option<String>,
    street_number: u32, //required; @TODO check address standards
    street: String,
    city: String,
    state: String,
    postcode: String, //to preserve any leading zeros (if allowed - TODO: check address standards if leadnig zeros are allowed)
}

pub fn addresses_to_result_with_csv_crate(bytes: &[u8]) -> Result<String, StatusCode> {
    let mut reader_builder = ReaderBuilder::default();
    reader_builder.has_headers(false); // counterintuitive: false means "include headers"

    let reader = reader_builder.from_reader(bytes);
    let mut csv_iter = reader.into_records();

    // We accept the CSV independent of its field order. Here we store a map of field names to their CSV field position (0-based).
    let column_names_owned;
    let mut field_to_column_idx = HashMap::<String, usize>::new();
    if let Some(header) = csv_iter.next() {
        if header.is_err() {
            // @TODO find better error statuses and include a message if needed; the same below
            return Err(StatusCode::NOT_ACCEPTABLE);
        } else {
            let header = header.unwrap();
            column_names_owned = (0..header.len())
                .map(|col_idx| header.get(col_idx))
                .collect::<Vec<_>>();

            if !(column_names_owned
                .iter()
                .all(|col_name_result| col_name_result.is_some()))
            {
                return Err(StatusCode::NOT_ACCEPTABLE);
            }
            column_names_owned
                .iter()
                .map(|col_name_result| col_name_result.unwrap().to_owned())
                .enumerate()
                .for_each(|(col_idx, col_name)| {
                    field_to_column_idx.insert(col_name, col_idx);
                });
            // @TODO there must be a macro to get a number (and list of) fields of a struct
            if field_to_column_idx.len() != 8 {
                return Err(StatusCode::NOT_ACCEPTABLE);
            }

            let mut expected_fields = vec![
                "reference",
                "address_type",
                "appt_suite_nu", // TODO Check why CSV crate shortens the field names!
                "street_number",
                "street",
                "city",
                "state",
                "postcode",
            ];
            expected_fields.sort();
            let mut actual_fields = field_to_column_idx
                .keys()
                .map(|field_name| field_name)
                .collect::<Vec<_>>();
            actual_fields.sort();
            if expected_fields != actual_fields {
                return Err(StatusCode::NOT_ACCEPTABLE);
            }
        }
    } else {
        // require the header
        return Err(StatusCode::NOT_ACCEPTABLE);
    }

    let col_name_to_idx = |col_name: &str| {
        let idx_option = field_to_column_idx.get(col_name).map(|&idx| idx);
        idx_option
    };

    // On nightly we could use #![allow(iterator_try_collect)] and reader.records().try_collect::<Vec<_>>();
    let mut addresses = vec![];

    for result in csv_iter {
        if result.is_err() {
            return Err(StatusCode::NOT_ACCEPTABLE);
        } else {
            let record = result.unwrap();

            let col_name_to_value = |col_name: &str| match col_name_to_idx(col_name) {
                None => Err(StatusCode::NOT_ACCEPTABLE),
                Some(idx) => record
                    .get(idx)
                    .map(|value| Ok(value))
                    .unwrap_or(Err(StatusCode::NOT_ACCEPTABLE)),
            };

            let address = Address {
                reference: col_name_to_value("reference")?.to_owned(),
                address_type: col_name_to_value("address_type")?.try_into()?,
                suite_number: {
                    match col_name_to_value("appt_suite_nu")?.trim() { //@TODO shortened field name - discuss
                        "" => None,
                        suite_number => Some(suite_number.to_owned()),
                    }
                },
                street_number: col_name_to_value("street_number")?
                    .parse::<u32>()
                    .or(Err(StatusCode::NOT_ACCEPTABLE))?,
                street: col_name_to_value("street")?.to_owned(),
                city: col_name_to_value("city")?.to_owned(),
                state: col_name_to_value("state")?.to_owned(),
                postcode: col_name_to_value("postcode")?.to_owned(),
            };
            addresses.push(address);
        }
    }
    let json = serde_json::to_string(&addresses);
    match json {
        Ok(json_string) => Ok(json_string),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn addresses_to_result_with_own_csv_parser(csv_content: String) -> Result<String, StatusCode> {
    let mut lines = csv_content.lines();
    let header = lines.next();
    if !header.is_some() {
        return Err(StatusCode::NOT_ACCEPTABLE);
    }
    let header = header.unwrap();
    let headings = header.split(',');
    // /headings.

    todo!()
}

#[cfg(test)]
mod test;
