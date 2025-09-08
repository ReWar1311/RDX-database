pub fn parse(input: &[u8]) -> Result<Vec<String>, String> {
    // Check if the input starts with RESP array marker '*'
    if input.starts_with(b"*") {
        return parse_resp_array(input);
    }
    
    // Fallback to simple text protocol
    let s = String::from_utf8_lossy(input);
    let parts: Vec<String> = s.trim()
        .split_whitespace()
        .map(|p| p.to_string())
        .collect();

    if parts.is_empty() {
        Err("Empty command".to_string())
    } else {
        Ok(parts)
    }
}

fn parse_resp_array(input: &[u8]) -> Result<Vec<String>, String> {
    let input_str = String::from_utf8_lossy(input);
    let mut parts = input_str.split("\r\n");
    
    // Parse array length
    let array_len = parts
        .next()
        .ok_or_else(|| "Invalid RESP array".to_string())?
        .trim_start_matches('*')
        .parse::<usize>()
        .map_err(|_| "Invalid array length".to_string())?;
    
    let mut result = Vec::with_capacity(array_len);
    
    // Parse each bulk string in the array
    for _ in 0..array_len {
        // Get bulk string length marker
        let length_marker = parts
            .next()
            .ok_or_else(|| "Incomplete RESP array".to_string())?;
        
        if !length_marker.starts_with('$') {
            return Err("Expected bulk string".to_string());
        }
        
        // Get actual string content
        let content = parts
            .next()
            .ok_or_else(|| "Incomplete bulk string".to_string())?;
        
        result.push(content.to_string());
    }
    
    Ok(result)
}

pub fn encode_simple(msg: &str) -> String {
    format!("+{}\r\n", msg)
}

pub fn encode_bulk(msg: &str) -> String {
    format!("${}\r\n{}\r\n", msg.len(), msg)
}

pub fn encode_nil() -> String {
    "$-1\r\n".to_string()
}
