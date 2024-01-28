

pub struct MessageType {
    pub(crate) value: String,
}

impl MessageType {
    pub fn from(s: &str) -> MessageType {
        MessageType { value: get_value(s).unwrap() }
    }
}

fn get_value(s: &str) -> Option<String> {
    let value= match s {
        "R" => "DEMANDE_COTATION",         // Demande de cotation
        "D" => "NOUVEL_ORDRE_ACHAT",      // Nouvel ordre - Achat
        "8" => "RAPPORT_EXECUTION",       // Rapport d'exécution
        "AK" => "CONFIRMATION_ORDRE",     // Confirmation d'ordre
        "0" => "BATTEMENT_DE_COEUR",      // Battement de cœur (Heartbeat)
        "1" => "DEMANDE_DE_TEST",         // Demande de test (Test Request)
        "2" => "DEMANDE_RENOI",           // Demande de renvoi de message (Resend Request)
        "4" => "REINITIALISATION_SEQUENCE", // Réinitialisation de séquence (Sequence Reset)
        "5" => "DECONNEXION",             // Déconnexion (Logout)
        "6" => "INDICATION_INTERET",      // Indication d'intérêt (Indication of Interest)
        "A" => "CONNEXION",               // Connexion (Logon)
        "AA" => "DEMANDE_DEFINITION_SECURITE", // Demande de définition de sécurité (Security Definition Request)
        "AE" => "DEFINITION_SECURITE",    // Définition de sécurité (Security Definition)
        "AF" => "DEMANDE_ETAT_SECURITE",  // Demande d'état de sécurité (Security Status Request)
        "AG" => "ETAT_SECURITE",          // État de sécurité (Security Status)
        _ => "UNDEFINED"
    };

    if value == "UNDEFINED" {
        return None
    }
    return Some(String::from(value));
}

