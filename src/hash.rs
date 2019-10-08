extern crate harsh;

use harsh::HarshBuilder;

//pub fn encode(cfg: &Config, id: usize) -> String {
pub fn encode(id: u64) -> Option<String> {
    let h = HarshBuilder::new()
        .salt("dinkysalt123")
        .length(10)
        .init()
        .unwrap();

    h.encode(&[id])
}

pub fn decode(hash: &str) -> Option<u64> {
    let h = HarshBuilder::new()
        .salt("dinkysalt123")
        .length(10)
        .init()
        .unwrap();

    match h.decode(&hash) {
        Some(s) => Some(s[0]),
        None => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(1), Some(String::from("adVJAB6Ej3")));
        assert_eq!(encode(2), Some(String::from("lDjr2br8Bm")));
        assert_eq!(encode(3), Some(String::from("ZjV6d06Mw2")));
        assert_eq!(encode(4), Some(String::from("1WPJnQvpNk")));
        assert_eq!(encode(5), Some(String::from("m876Z3r3yM")));
        assert_eq!(encode(std::u64::MAX), Some(String::from("aOE3eljoeG2zO")));
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("adVJAB6Ej3"), Some(1));
        assert_eq!(decode("lDjr2br8Bm"), Some(2));
        assert_eq!(decode("ZjV6d06Mw2"), Some(3));
        assert_eq!(decode("1WPJnQvpNk"), Some(4));
        assert_eq!(decode("m876Z3r3yM"), Some(5));
        assert_eq!(decode("aOE3eljoeG2zO"), Some(std::u64::MAX));
    }

}