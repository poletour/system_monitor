use system_monitor::meminfo::{MeminfoParseError, parse_meminfo};
use system_monitor::system;

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
fn parse_meminfo_with_only_memtotal() {
    let input = "MemTotal:       15911112 kB";
    let result = parse_meminfo(input);
    let stats = result.unwrap();
    assert_eq!(stats.total, 15911112);
    assert_eq!(stats.free, None);
    assert_eq!(stats.available, None);
}

#[test]
fn parse_meminfo_ignores_unrelated_lines() {
    let input = concat!(
        "MemTotal:       15911096 kB\n",
        "MemFree:        13480996 kB\n",
        "MemAvailable:   13670284 kB\n",
        "Buffers:            3652 kB\n",
        "Cached:           345272 kB\n",
        "SwapCached:            0 kB\n",
        "Active:            77704 kB\n",
        "Inactive:        1929952 kB\n",
        "Active(anon):       2660 kB\n",
        "Inactive(anon):  1659668 kB\n",
        "Active(file):      75044 kB\n",
        "Inactive(file):   270284 kB"
    );

    let result = parse_meminfo(input);
    let stats = result.unwrap();
    assert_eq!(stats.total, 15911096);
    assert_eq!(stats.free, Some(13480996));
    assert_eq!(stats.available, Some(13670284));
}

#[test]
fn parse_meminfo_accepts_fields_in_different_order() {
    let input = concat!(
        "MemAvailable:   13670284 kB\n",
        "MemFree:        13480996 kB\n",
        "MemTotal:       15911096 kB"
    );

    let result = parse_meminfo(input);
    let stats = result.unwrap();
    assert_eq!(stats.total, 15911096);
    assert_eq!(stats.free, Some(13480996));
    assert_eq!(stats.available, Some(13670284));
}

#[test]
fn parse_meminfo_empty_content_returns_missing_memtotal() {
    let input = "";
    let result = parse_meminfo(input);
    assert_eq!(result, Err(MeminfoParseError::MissingMemTotal));
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