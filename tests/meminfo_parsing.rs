use system_monitor::meminfo::{MeminfoParseError, parse_meminfo};

#[test]
fn parse_meminfo_valid_content() {
    // 1. Construire une fausse string qui ressemble à /proc/meminfo
    let input = concat!(
        "MemTotal:       15911112 kB\n",
        "MemFree:        12697056 kB\n",
        "MemAvailable:   13927080 kB\n",
    );

    // 2. Appeler parse_meminfo(...)
    let result = parse_meminfo(input);

    // 3. Vérifier le résultat avec assert_eq!
    let stats = result.unwrap();
    assert_eq!(stats.total, 15911112);
    assert_eq!(stats.free, Some(12697056));
    assert_eq!(stats.available, Some(13927080));    
}

#[test]
fn parse_meminfo_missing_memtotal() {
    let input = concat!(
       "MemFree:        12697056 kB\n",
       "MemAvailable:   13927080 kB\n",
    );

    let result = parse_meminfo(input);

    assert_eq!(result, Err(MeminfoParseError::MissingMemTotal));
}