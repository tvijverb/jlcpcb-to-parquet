pub fn parse_inductance_description(component_description: &str) -> Option<f64> {
    let mut inductance_value: Option<f64> = None;

    let description_parts: Vec<&str> = component_description.split(' ').collect();
    for part in description_parts {
        if part.ends_with("pH") {
            let numeric_part = part.trim_end_matches("pH");
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value);
                break;
            }
        } else if part.ends_with("nH") {
            let numeric_part = part.trim_end_matches("nH");
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value * 1_000.0);
                break;
            }
        } else if part.ends_with("uH") {
            let numeric_part = part.trim_end_matches("uH");
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value * 1_000_000.0);
                break;
            }
        } else if part.ends_with("µH") {
            let numeric_part = part.trim_end_matches("µH");
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value * 1_000_000.0);
                break;
            }
        } else if part.ends_with("mH") {
            let numeric_part = part.trim_end_matches("mH");
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value * 1_000_000_000.0);
                break;
            }
        } else if part.ends_with("kH") {
            let numeric_part = part.trim_end_matches("kH");
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value * 1_000_000_000_000_000.0);
                break;
            }
        } else if part.ends_with("MH") {
            let numeric_part = part.trim_end_matches("MH");
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value * 1_000_000_000_000_000_000.0);
                break;
            }
        } else if part.ends_with('H') {
            let numeric_part = part.trim_end_matches('H');
            if let Ok(value) = numeric_part.parse::<f64>() {
                inductance_value = Some(value * 1_000_000_000_000.0);
                break;
            }
        }
    }
    inductance_value
}
