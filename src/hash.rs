extern crate harsh;

use harsh::HarshBuilder;

use crate::conf::Conf;

pub fn encode(conf: &Conf, id: i32) -> Option<String> {
    let h = HarshBuilder::new()
        .salt(conf.hash.salt.clone())
        .length(conf.hash.length)
        .init()
        .unwrap();

    h.encode(&[id as u64])
}

pub fn decode(conf: &Conf, hash: &str) -> Option<i32> {
    let h = HarshBuilder::new()
        .salt(conf.hash.salt.clone())
        .length(conf.hash.length)
        .init()
        .unwrap();

    match h.decode(&hash) {
        Some(s) => Some(s[0] as i32),
        None => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let c = Conf::default();
        assert_eq!(encode(&c, 0), Some(String::from("7ma6kjJ8Aj")));
        assert_eq!(encode(&c, 1), Some(String::from("adVJAB6Ej3")));
        assert_eq!(encode(&c, 2), Some(String::from("lDjr2br8Bm")));
        assert_eq!(encode(&c, 3), Some(String::from("ZjV6d06Mw2")));
        assert_eq!(encode(&c, 4), Some(String::from("1WPJnQvpNk")));
        assert_eq!(encode(&c, 5), Some(String::from("m876Z3r3yM")));
        assert_eq!(encode(&c, std::i32::MAX), Some(String::from("V6dKV22glJ")));
    }

    #[test]
    fn test_decode() {
        let c = Conf::default();
        assert_eq!(decode(&c, "adVJAB6Ej3"), Some(1));
        assert_eq!(decode(&c, "lDjr2br8Bm"), Some(2));
        assert_eq!(decode(&c, "ZjV6d06Mw2"), Some(3));
        assert_eq!(decode(&c, "1WPJnQvpNk"), Some(4));
        assert_eq!(decode(&c, "m876Z3r3yM"), Some(5));
        assert_eq!(decode(&c, "V6dKV22glJ"), Some(std::i32::MAX));
    }

}
