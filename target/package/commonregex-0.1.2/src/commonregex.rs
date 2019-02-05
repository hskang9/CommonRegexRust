extern crate regex;
use regex::Regex;


pub const   DATE           :&str= r#"(?i)(?:[0-3]?\d(?:st|nd|rd|th)?\s+(?:of\s+)?(?:jan\.?|january|feb\.?|february|mar\.?|march|apr\.?|april|may|jun\.?|june|jul\.?|july|aug\.?|august|sep\.?|september|oct\.?|october|nov\.?|november|dec\.?|december)|(?:jan\.?|january|feb\.?|february|mar\.?|march|apr\.?|april|may|jun\.?|june|jul\.?|july|aug\.?|august|sep\.?|september|oct\.?|october|nov\.?|november|dec\.?|december)\s+[0-3]?\d(?:st|nd|rd|th)?)(?:,)?\s*(?:\d{4})?|[0-3]?\d[-\./][0-3]?\d[-\./]\d{2,4}"#;
pub const	TIME           :&str= r#"(?i)\d{1,2}:\d{2} ?(?:[ap]\.?m\.?)?|\d[ap]\.?m\.?"#;
pub const	PHONE          :&str= r#"(?:(?:\+?\d{1,3}[-.\s*]?)?(?:\(?\d{3}\)?[-.\s*]?)?\d{3}[-.\s*]?\d{4,6})|(?:(?:(?:\(\+?\d{2}\))|(?:\+?\d{2}))\s*\d{2}\s*\d{3}\s*\d{4})"#;
pub const	PHONEWTHEXT    :&str= r#"(?i)(?:(?:\+?1\s*(?:[.-]\s*)?)?(?:\(\s*(?:[2-9]1[02-9]|[2-9][02-8]1|[2-9][02-8][02-9])\s*\)|(?:[2-9]1[02-9]|[2-9][02-8]1|[2-9][02-8][02-9]))\s*(?:[.-]\s*)?)?(?:[2-9]1[02-9]|[2-9][02-9]1|[2-9][02-9]{2})\s*(?:[.-]\s*)?(?:[0-9]{4})(?:\s*(?:#|x\.?|ext\.?|extension)\s*(?:\d+)?)"#;
pub const	LINK           :&str= r#"(http(s)?:\\/\\/.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)"#;
pub const	EMAIL          :&str= r#"(?i)([A-Za-z0-9!#$%&'*+\\/=?^_{|.}~-]+@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?)"#;
pub const	IPV4           :&str= r#"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)"#;
pub const	IPV6           :&str= r#"(?:(?:(?:[0-9A-Fa-f]{1,4}:){7}(?:[0-9A-Fa-f]{1,4}|:))|(?:(?:[0-9A-Fa-f]{1,4}:){6}(?::[0-9A-Fa-f]{1,4}|(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(?:(?:[0-9A-Fa-f]{1,4}:){5}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,2})|:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(?:(?:[0-9A-Fa-f]{1,4}:){4}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,3})|(?:(?::[0-9A-Fa-f]{1,4})?:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){3}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,4})|(?:(?::[0-9A-Fa-f]{1,4}){0,2}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){2}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,5})|(?:(?::[0-9A-Fa-f]{1,4}){0,3}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?:(?:[0-9A-Fa-f]{1,4}:){1}(?:(?:(?::[0-9A-Fa-f]{1,4}){1,6})|(?:(?::[0-9A-Fa-f]{1,4}){0,4}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(?::(?:(?:(?::[0-9A-Fa-f]{1,4}){1,7})|(?:(?::[0-9A-Fa-f]{1,4}){0,5}:(?:(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(?:\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(?:%.+)?\s*"#;
pub const	PRICE          :&str= r#"[$]\s?[+-]?[0-9]{1,3}(?:(?:,?[0-9]{3}))*(?:\.[0-9]{1,2})?"#;
pub const	HEXCOLOR       :&str= r#"(?:#?([0-9a-fA-F]{6}|[0-9a-fA-F]{3}))"#;
pub const	CREDITCARD     :&str= r#"(?:(?:(?:\d{4}[- ]?){3}\d{4}|\d{15,16}))"#;
pub const	VISA           :&str= r#"4\d{3}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}"#;
pub const	MASTERCARD     :&str= r#"5[1-5]\d{2}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}"#;
pub const	BTCADDRESS     :&str= r#"[13][a-km-zA-HJ-NP-Z1-9]{25,34}"#;
pub const	STREETADDRESS  :&str= r#"\d{1,4} [\w\s]{1,20}(?:street|st|avenue|ave|road|rd|highway|hwy|square|sq|trail|trl|drive|dr|court|ct|park|parkway|pkwy|circle|cir|boulevard|blvd)\W?"#;
pub const	ZIPCODE        :&str= r#"\b\d{5}(?:[-\s]\d{4})?\b"#;
pub const	POBOX          :&str= r#"(?i)P\.? ?O\.? Box \d+"#;
pub const	SSN            :&str= r#"(?:\d{3}-\d{2}-\d{4})"#;
pub const	MD5HEX         :&str= r#"[0-9a-fA-F]{32}"#;
pub const	SHA1HEX        :&str= r#"[0-9a-fA-F]{40}"#;
pub const	SHA256HEX      :&str= r#"[0-9a-fA-F]{64}"#;
pub const	GUID           :&str= r#"[0-9a-fA-F]{8}-?[a-fA-F0-9]{4}-?[a-fA-F0-9]{4}-?[a-fA-F0-9]{4}-?[a-fA-F0-9]{12}"#;
pub const	ISBN13         :&str= r#"(?:[\d]-?){12}[\dxX]"#;
pub const	ISBN10         :&str= r#"(?:[\d]-?){9}[\dxX]"#;
pub const	MACADDRESS     :&str= r#"(([a-fA-F0-9]{2}[:-]){5}([a-fA-F0-9]{2}))"#;
pub const	IBAN           :&str= r#"[A-Z]{2}\d{2}[A-Z0-9]{4}\d{7}([A-Z\d]?){0,16}"#;
pub const	GITREPO        :&str= r#"((git|ssh|http(s)?)|(git@[\w\.]+))(:(\\/\\/)?)([\w\.@\\:/\-~]+)(\.git)(\\/)?"#;


#[derive(Debug)]
pub struct CommonRegex<'a> {
    pub dates: Vec<&'a str>,
    pub times: Vec<&'a str>,
    pub phones: Vec<&'a str>,
    pub phones_with_exts: Vec<&'a str>,
    pub links: Vec<&'a str>,
    pub emails: Vec<&'a str>,
    pub ipv4s: Vec<&'a str>,
    pub ipv6s: Vec<&'a str>,
    pub prices: Vec<&'a str>,
    pub hex_colors: Vec<&'a str>,
    pub credit_cards: Vec<&'a str>,
    pub visas: Vec<&'a str>,
    pub mastercards: Vec<&'a str>,
    pub btc_addresses: Vec<&'a str>,
    pub street_addresses: Vec<&'a str>,
    pub zip_codes: Vec<&'a str>,
    pub po_boxs: Vec<&'a str>,
    pub ssns: Vec<&'a str>,
    pub md5s: Vec<&'a str>,
    pub sha1s: Vec<&'a str>,
    pub sha2s: Vec<&'a str>,
    pub guids: Vec<&'a str>,
    pub isbn13s: Vec<&'a str>,
    pub isbn10s: Vec<&'a str>,
    pub mac_addresses: Vec<&'a str>,
    pub ibans: Vec<&'a str>,
    pub gitrepos: Vec<&'a str>,
 }

impl <'a>CommonRegex<'a> {
    pub fn new() -> CommonRegex<'a> {
        CommonRegex {
                dates: Vec::new(),
                times: Vec::new(),
                phones: Vec::new(),
                phones_with_exts: Vec::new(),
                links: Vec::new(),
                emails: Vec::new(),
                ipv4s: Vec::new(),
                ipv6s: Vec::new(),
                prices: Vec::new(),
                hex_colors: Vec::new(),
                credit_cards: Vec::new(),
                visas: Vec::new(),
                mastercards: Vec::new(),
                btc_addresses: Vec::new(),
                street_addresses: Vec::new(),
                zip_codes: Vec::new(),
                po_boxs: Vec::new(),
                ssns: Vec::new(),
                md5s: Vec::new(),
                sha1s: Vec::new(),
                sha2s: Vec::new(),
                guids: Vec::new(),
                isbn13s: Vec::new(),
                isbn10s: Vec::new(),
                mac_addresses: Vec::new(),
                ibans: Vec::new(),
                gitrepos: Vec::new(),
        }
    }
    pub fn CommonRegex(&self, text: &'a str) -> CommonRegex<'a> {
        CommonRegex {
                dates: dates(text),
                times: times(text),
                phones: phones(text),
                phones_with_exts: phones_with_exts(text),
                links: links(text),
                emails: emails(text),
                ipv4s: ips(text),
                ipv6s: ipv6s(text),
                prices: prices(text),
                hex_colors: hex_colors(text),
                credit_cards: credit_cards(text),
                visas: visas(text),
                mastercards: mastercards(text),
                btc_addresses: btc_addresses(text),
                street_addresses: street_addresses(text),
                zip_codes: zip_codes(text),
                po_boxs: po_boxs(text),
                ssns: ssns(text),
                md5s: md5s(text),
                sha1s: sha1s(text),
                sha2s: sha2s(text),
                guids: guids(text),
                isbn13s: isbn13s(text),
                isbn10s: isbn10s(text),
                mac_addresses: mac_addresses(text),
                ibans: ibans(text),
                gitrepos: gitrepos(text),
        }
    }
}


pub fn CommonRegex<'a>(text: &'a str) -> CommonRegex<'a> {
        CommonRegex {
                dates: dates(text),
                times: times(text),
                phones: phones(text),
                phones_with_exts: phones_with_exts(text),
                links: links(text),  // FIXME: Regex parse error
                emails: emails(text),
                ipv4s: ips(text),
                ipv6s: ipv6s(text),
                prices: prices(text),
                hex_colors: hex_colors(text),
                credit_cards: credit_cards(text),
                visas: visas(text),
                mastercards: mastercards(text),
                btc_addresses: btc_addresses(text),
                street_addresses: street_addresses(text),
                zip_codes: zip_codes(text),
                po_boxs: po_boxs(text),
                ssns: ssns(text),
                md5s: md5s(text),
                sha1s: sha1s(text),
                sha2s: sha2s(text),
                guids: guids(text),
                isbn13s: isbn13s(text),
                isbn10s: isbn10s(text),
                mac_addresses: mac_addresses(text),
                ibans: ibans(text),
                gitrepos: gitrepos(text)
        }
    }


#[allow(dead_code)]
pub fn parse<'caps>(regex: &str, text: &'caps str) -> Vec<&'caps str> {
    let mut caps:Vec<&str> = [].to_vec();
    for cap in Regex::new(regex).unwrap().captures_iter(text) {
        caps.push(cap.get(0).map_or("", |m| m.as_str()));
    }
    caps
}

#[allow(dead_code)]
pub fn dates(text: &str) -> Vec<&str> {
   parse(DATE, text)
}

#[allow(dead_code)]
pub fn times(text: &str) -> Vec<&str> {
    parse(TIME, text)
}

#[allow(dead_code)]
pub fn phones(text: &str) -> Vec<&str> {
    parse(PHONE, text)
}

#[allow(dead_code)]
pub fn phones_with_exts(text: &str) -> Vec<&str> {
    parse(PHONEWTHEXT, text)
}

#[allow(dead_code)]
pub fn links(text: &str) -> Vec<&str> {
    parse(LINK, text)
}

#[allow(dead_code)]
pub fn emails(text: &str) -> Vec<&str> {
    parse(EMAIL, text)
}

#[allow(dead_code)]
pub fn ips(text: &str) -> Vec<&str> {
    parse(IPV4, text)
}

#[allow(dead_code)]
pub fn ipv6s(text: &str) -> Vec<&str> {
    parse(IPV6, text)
}

#[allow(dead_code)]
pub fn prices(text: &str) -> Vec<&str> {
    parse(PRICE, text)
}

#[allow(dead_code)]
pub fn hex_colors(text: &str) -> Vec<&str> {
    parse(HEXCOLOR, text)
}

#[allow(dead_code)]
pub fn credit_cards(text: &str) -> Vec<&str> {
    parse(CREDITCARD, text)
}

#[allow(dead_code)]
pub fn visas(text: &str) -> Vec<&str> {
    parse(VISA, text)
}

#[allow(dead_code)]
pub fn mastercards(text: &str) -> Vec<&str> {
    parse(MASTERCARD, text)
}

#[allow(dead_code)]
pub fn btc_addresses(text: &str) -> Vec<&str> {
    parse(BTCADDRESS, text)    
}

#[allow(dead_code)]
pub fn street_addresses(text: &str) -> Vec<&str> {
    parse(STREETADDRESS, text)
}

#[allow(dead_code)]
pub fn zip_codes(text: &str) -> Vec<&str> {
    parse(ZIPCODE, text)
}

#[allow(dead_code)]
pub fn po_boxs(text: &str) -> Vec<&str> {
    parse(POBOX, text)
}

#[allow(dead_code)]
pub fn ssns(text: &str) -> Vec<&str> {
    parse(SSN, text)
}

#[allow(dead_code)]
pub fn md5s(text: &str) -> Vec<&str> {
    parse(MD5HEX, text)
}

#[allow(dead_code)]
pub fn sha1s(text: &str) -> Vec<&str> {
    parse(SHA1HEX, text)
}

#[allow(dead_code)]
pub fn sha2s(text: &str) -> Vec<&str> {
    parse(SHA256HEX, text)
}

#[allow(dead_code)]
pub fn guids(text: &str) -> Vec<&str> {
    parse(GUID, text)
}

#[allow(dead_code)]
pub fn isbn13s(text: &str) -> Vec<&str> {
    parse(ISBN13, text)
}

#[allow(dead_code)]
pub fn isbn10s(text: &str) -> Vec<&str> {
    parse(ISBN10, text)
}

#[allow(dead_code)]
pub fn mac_addresses(text: &str) -> Vec<&str> {
    parse(MACADDRESS, text)
}

#[allow(dead_code)]
pub fn ibans(text: &str) -> Vec<&str> {
    parse(IBAN, text)
}

#[allow(dead_code)]
pub fn gitrepos(text: &str) -> Vec<&str> {
    parse(GITREPO, text)
}







