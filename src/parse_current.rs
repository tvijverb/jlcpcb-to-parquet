pub fn parse_current_description(component_description: &str) -> Option<f64> {
    let mut current_value: Option<f64> = None;

    let description_parts: Vec<&str> = component_description.split(' ').collect();
    for part in description_parts {
        if part.ends_with("pA") {
            let numeric_part = part.trim_end_matches("pA");
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value);
                break;
            }
        } else if part.ends_with("nA") {
            let numeric_part = part.trim_end_matches("nA");
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value * 1e3);
                break;
            }
        } else if part.ends_with("uA") {
            let numeric_part = part.trim_end_matches("uA");
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value * 1e6);
                break;
            }
        } else if part.ends_with("μA") {
            let numeric_part = part.trim_end_matches("μA");
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value * 1e6);
                break;
            }  
        } else if part.ends_with("mA") {
            let numeric_part = part.trim_end_matches("mA");
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value * 1e9);
                break;
            }
        } else if part.ends_with("kA") {
            let numeric_part = part.trim_end_matches("kA");
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value * 1e15);
                break;
            }
        } else if part.ends_with("MA") {
            let numeric_part = part.trim_end_matches("MA");
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value * 1e18);
                break;
            }
        } else if part.ends_with('A') {
            let numeric_part = part.trim_end_matches('A');
            if let Ok(value) = numeric_part.parse::<f64>() {
                current_value = Some(value * 1e12);
                break;
            }
        }
    }
    current_value
}