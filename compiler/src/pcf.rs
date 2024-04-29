use regex::Regex;
use std::collections::HashMap;

pub struct PcfFile {
    map: HashMap<String, u32>,
}

pub fn parse_pcf(input: &str) -> PcfFile {
    let mut pcf = HashMap::new();
    let re = Regex::new(r"(?m)^set_io (\S+) (\d+)$").unwrap();
    // we run a captures on it.
    for (_, [name, pin]) in re.captures_iter(input).map(|c| c.extract()) {
        if pcf.contains_key(name) {
            panic!("name collision in pcf file");
        }
        let num: u32 = str::parse(pin).unwrap();
        pcf.insert(name.to_string(), num);
    }
    PcfFile { map: pcf }
}

impl PcfFile {
    pub fn pin(&self, name: &str) -> Option<u32> {
        self.map.get(name).cloned()
    }
    pub fn pinvec(&self, name: &str, index: usize) -> Option<u32> {
        // construct a name of the form <name>[<index>]
        let realname = format!("{name}[{index}]");
        self.map.get(&realname).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let test_str = "set_io pinName 1";
        let name_expected = "pinName";
        let pin_expected = 1;
        let f = parse_pcf(test_str);
        assert_eq!(f.pin(name_expected), Some(pin_expected));
        assert_eq!(f.pin("invalid"), None);
    }

    #[test]
    fn test_pins() {
        let test = "
set_io scalar 1
set_io vec[0] 2
set_io vec[1] 3";
        let f = parse_pcf(test);
        assert_eq!(f.pin("scalar"), Some(1));
        assert_eq!(f.pinvec("vec", 0), Some(2));
        assert_eq!(f.pinvec("vec", 1), Some(3));
        assert_eq!(f.pinvec("vec", 2), None);
    }

    #[test]
    #[should_panic]
    fn test_name_collision() {
        let test = "
set_io scalar 1
set_io scalar 2";
        parse_pcf(test);
    }
}
