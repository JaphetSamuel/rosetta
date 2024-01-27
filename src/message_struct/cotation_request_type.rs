struct CotationRequestType { value: String}

impl CotationRequestType {

    pub fn from(s: &str) {

    }
}

fn get_value(s: &str) -> String {

    let value = match s {
        "2" => "LIMIT",
        _ => "UNDEFINED"
    };

    return String::from(value);
}