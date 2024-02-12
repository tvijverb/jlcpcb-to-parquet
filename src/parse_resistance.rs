pub fn parse_resistance_description(component_description: &str) -> Option<f64> {
    let mut resistance_value: Option<f64> = None;

    let description_parts: Vec<&str> = component_description.split(' ').collect();
    for part in description_parts {
        if part.ends_with("pΩ") {
            let numeric_part = part.trim_end_matches("pΩ");
            if let Ok(value) = numeric_part.parse::<f64>() {
                resistance_value = Some(value);
                break;
            }
        } 
        else if part.ends_with("nΩ") {
            let numeric_part = part.trim_end_matches("nΩ");
            if let Ok(value) = numeric_part.parse::<f64>() {
                resistance_value = Some(value*1_000.0);
                break;
            }
        } else if part.ends_with("uΩ") {
            let numeric_part = part.trim_end_matches("uΩ");
            if let Ok(value) = numeric_part.parse::<f64>() {
                resistance_value = Some(value*1_000_000.0);
                break;
            }
        } else if part.ends_with("mΩ") {
            let numeric_part = part.trim_end_matches("mΩ");
            if let Ok(value) = numeric_part.parse::<f64>() {
                resistance_value = Some(value*1_000_000_000.0);
                break;
            }
        } else if part.ends_with("kΩ") {
            let numeric_part = part.trim_end_matches("kΩ");
            if let Ok(value) = numeric_part.parse::<f64>() {
                resistance_value = Some(value*1_000_000_000_000_000.0);
                break;
            }
        } else if part.ends_with("MΩ") {
            let numeric_part = part.trim_end_matches("MΩ");
            if let Ok(value) = numeric_part.parse::<f64>() {
                resistance_value = Some(value*1_000_000_000_000_000_000.0);
                break;
            }
        } else if part.ends_with('Ω') {
            let numeric_part = part.trim_end_matches('Ω');
            if let Ok(value) = numeric_part.parse::<f64>() {
                resistance_value = Some(value*1_000_000_000_000.0);
                break;
            }
        } 
    }
    resistance_value
}