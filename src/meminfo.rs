#[derive(Debug, PartialEq)]
pub struct MemoryStats {
    pub total: u64, 
    pub free: Option<u64>,
    pub available: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub enum MeminfoParseError {
    MissingMemTotal,
}

// Parcourir chaque ligne du fichier
pub fn parse_meminfo(content: &str) -> Result<MemoryStats, MeminfoParseError> {
    
    let mut mem_total = None;
    let mut mem_free = None;
    let mut mem_available = None;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            mem_total = parse_value_token(line);
        } else if line.starts_with("MemFree:") {
            mem_free = parse_value_token(line);
        } else if line.starts_with("MemAvailable:") {
            mem_available = parse_value_token(line);
        }
    }

    let total: u64 = match mem_total {
        Some(v) => {v}
        None => {return Err(MeminfoParseError::MissingMemTotal)}
    };

    let meminfo = MemoryStats{total: total, free: mem_free, available: mem_available};

    Ok(meminfo)
}

fn parse_value_token(line: &str) -> Option<u64> {
        let mut it= line.split_whitespace();
        let _key = it.next()?;
        let value_str = it.next()?;

    value_str.parse::<u64>().ok()
}  