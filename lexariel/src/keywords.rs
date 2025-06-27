//! keyword and directive recognition

/// recognized keyword (instruction)
pub(crate) fn is_keyword(word: &str) -> bool {
    matches!(word.to_uppercase().as_str(),
        "DOD" | "ODE" | "ŁAD" | "LAD" | "POB" | "SOB" | "SOM" | "SOZ" | "STP" | 
        "DNS" | "PZS" | "SDP" | "CZM" | "MSK" | "PWR" | "WPR" | "WEJSCIE" | "WYJSCIE" | "WYJ"
    )
}

/// recognized directive
pub(crate) fn is_directive(word: &str) -> bool {
    matches!(word.to_uppercase().as_str(),
        "RST" | "RPA" | "MAKRO" | "KONM" | "NAZWA_LOKALNA"
    )
}
