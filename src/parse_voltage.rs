pub fn parse_voltage_description(component_description: &str) -> Option<f64> {
    let mut voltage_value: Option<f64> = None;

    let description_parts: Vec<&str> = component_description.split(' ').collect();
    for part in description_parts {
        if part.ends_with("pV") {
            let numeric_part = part.trim_end_matches("pV");
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value);
                break;
            }
        } else if part.ends_with("nV") {
            let numeric_part = part.trim_end_matches("nV");
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value * 1e3);
                break;
            }
        } else if part.ends_with("uV") {
            let numeric_part = part.trim_end_matches("uV");
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value * 1e6);
                break;
            }
        } else if part.ends_with("μV") {
            let numeric_part = part.trim_end_matches("μV");
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value * 1e6);
                break;
            }  
        } else if part.ends_with("mV") {
            let numeric_part = part.trim_end_matches("mV");
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value * 1e9);
                break;
            }
        } else if part.ends_with("kV") {
            let numeric_part = part.trim_end_matches("kV");
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value * 1e15);
                break;
            }
        } else if part.ends_with("MV") {
            let numeric_part = part.trim_end_matches("MV");
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value * 1e18);
                break;
            }
        } else if part.ends_with('V') {
            let numeric_part = part.trim_end_matches('V');
            if let Ok(value) = numeric_part.parse::<f64>() {
                voltage_value = Some(value * 1e12);
                break;
            }
        }
    }
    voltage_value
}