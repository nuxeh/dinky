extern crate harsh;

use harsh::HarshBuilder;

//pub fn encode(cfg: &Config, id: usize) -> String {
pub fn encode(id: u64) -> String {
    let h = HarshBuilder::new()
        .salt("dinkysalt123")
        .length(10)
        .init()
        .unwrap();

    h.encode(&[id]).unwrap()
}

pub fn decode(hash: &str) -> u64 {
    let h = HarshBuilder::new()
        .salt("dinkysalt123")
        .length(10)
        .init()
        .unwrap();

    h.decode(&hash).unwrap()[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(1), String::from("adVJAB6Ej3"));
        assert_eq!(encode(2), String::from("lDjr2br8Bm"));
        assert_eq!(encode(3), String::from("ZjV6d06Mw2"));
        assert_eq!(encode(4), String::from("1WPJnQvpNk"));
        assert_eq!(encode(5), String::from("m876Z3r3yM"));
        assert_eq!(encode(std::u64::MAX), String::from("aOE3eljoeG2zO"));
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("adVJAB6Ej3"), 1);
        assert_eq!(decode("lDjr2br8Bm"), 2);
        assert_eq!(decode("ZjV6d06Mw2"), 3);
        assert_eq!(decode("1WPJnQvpNk"), 4);
        assert_eq!(decode("m876Z3r3yM"), 5);
        assert_eq!(decode("aOE3eljoeG2zO"), std::u64::MAX);
    }

}
