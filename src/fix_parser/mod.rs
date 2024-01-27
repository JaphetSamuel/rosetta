use std::any::Any;
use std::collections::HashMap;
use crate::message_struct::message_type;
pub fn parse(message: String) {
    let mut result: HashMap<&str,&str> = HashMap::new();

    let e: Vec<&str> = message.split('|').into_iter().collect();

    for section in e.iter() {
        let tandem: Vec<&str> = section.split('=').into_iter().collect();

        if tandem.len() == 2 {
            result.insert(tandem[0], tandem[1]);
        }
        else {
            result.insert(tandem[0], "neant");
        }
    }

    result.iter().for_each(|(k,v)| print!("{} : {v} \n", tag_to_human(k)));

    print!("//////////////////////////////////////////////////////////// \n")
}

pub fn tag_to_human(tag : &str) -> &str {
    let val = match tag {
        "8" => "version",
        "35" => "type",
        "34" => "numero sequence",
        "49" => "sender id",
        "56" => "id destinataire",
        "11" => "identifiant unique",
        "55" => "Symbole instrument financier",
        "54" => "coté",
        "38" => "quantité",
        "40" => "type demande",
        "" => "neant",
        _ => tag
    };

    return val
}

pub fn val_to_human(k: &str, val:&str) -> Box<dyn Any> {
    let result = match k {
        "35" => Box::from(message_type::MessageType::from(val).value),
        _ => Box::from(String::from(val))
    };

    return result;
}