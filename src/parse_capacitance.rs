pub fn parse_capacitance_description(component_description: &str) -> Option<f64> {
    let mut capacitance_value: Option<f64> = None;

    let description_parts: Vec<&str> = component_description.split(' ').collect();
    for part in description_parts {
        if part.ends_with("pF") {
            let numeric_part = part.trim_end_matches("pF");
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value);
                break;
            }
        } else if part.ends_with("nF") {
            let numeric_part = part.trim_end_matches("nF");
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value * 1_000.0);
                break;
            }
        } else if part.ends_with("uF") {
            let numeric_part = part.trim_end_matches("uF");
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value * 1_000_000.0);
                break;
            }
        } else if part.ends_with("µF") {
            let numeric_part = part.trim_end_matches("µF");
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value * 1_000_000.0);
                break;
            }
        } else if part.ends_with("mF") {
            let numeric_part = part.trim_end_matches("mF");
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value * 1_000_000_000.0);
                break;
            }
        } else if part.ends_with("kF") {
            let numeric_part = part.trim_end_matches("kF");
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value * 1_000_000_000_000_000.0);
                break;
            }
        } else if part.ends_with("MF") {
            let numeric_part = part.trim_end_matches("MF");
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value * 1_000_000_000_000_000_000.0);
                break;
            }
        } else if part.ends_with('F') {
            let numeric_part = part.trim_end_matches('F');
            if let Ok(value) = numeric_part.parse::<f64>() {
                capacitance_value = Some(value * 1_000_000_000_000.0);
                break;
            }
        }
    }
    capacitance_value
}
