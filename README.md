CommonRegexRust
=============

[CommonRegex](https://github.com/madisonmay/CommonRegex/ "CommonRegex") port for Rust

Find a lot of kinds of common information in a string.

Pull requests welcome!

Please note that this is currently English/US specific.

[![Build Status](https://travis-ci.org/talyssonoc/commonregexjs.svg?branch=master)](https://travis-ci.org/hskang9/CommonRegexRust)

Installation
===
install via Cargo with

```sh
cargo install commonregex
```

Or use Crates.toml

```
[package]
name = <package-name>
version = <version>
authors = [<you>]

[dependencies]
commonregex = "0.1.0"
```
then run `cargo build` on terminal.

API
===

You can instantiate a CommonRegex object passing a string in the constructor and use the fields of the object to acess the matches and the methods for the matches of other strings (passing the string as parameter), or not pass a string in the constructor and just use the methods.

Possible properties and its equivalent methods:

* dates: `dates(text)`                            
* times: `times(text: &str)`
* phones: `phones(text: &str)`
* phones_with_exts: `phones_with_exts(text: &str)`
* links: `links(text: &str)`
* emails: `emails(text: &str)`
* ipv4s: `ips(text: &str)`
* ipv6s: `ipv6s(text: &str)`
* prices: `prices(text: &str)`
* hex_colors: `hex_colors(text: &str)`
* credit_cards: `credit_cards(text: &str)`
* visas: `visas(text: &str)`
* mastercards: `mastercards(text: &str)`
* btc_addresses: `btc_addresses(text: &str)`
* street_addresses: `street_addresses(text: &str)`
* zip_codes: `zip_codes(text: &str)`
* po_boxs: `po_boxs(text: &str)`
* ssns: `ssns(text: &str)`
* md5s: `md5s(text: &str)`
* sha1s: `sha1s(text: &str)`
* sha2s: `sha2s(text: &str)`
* guids: `guids(text: &str)`
* isbn13s: `isbn13s(text: &str)`
* isbn10s: `isbn10s(text: &str)`
* mac_addresses: `mac_addresses(text: &str)`
* ibans: `ibans(text: &str)`
* gitrepos: `gitrepos(text: &str)`
* All of above: `CommonRegex(text: &str)`
* Custom: `parse(regex: &str, text: &str)`

Examples
========
```
extern crate commonregex;

let text = 'John, please get that article on www.linkedin.com to me by 5:00PM 
on Jan 9th 2012. 4:00 would be ideal, actually. If you have any 
questions, You can reach me at (519)-236-2723x341 or get in touch with
my associate at harold.smith@gmail.com';

let parsed =  commonregex::CommonRegex(text);
println!("{:?}", parsed);
/* prints CommonRegex { dates: ["Jan 9th 2012"], times: ["5:00PM", "4:00 "], phones: ["(519)-236-2723"], phones_with_exts: ["(519)-236-2723x341"], links: ["www.linkedin.com", "harold.smith@gmail.com"], emails: ["harold.smith@gmail.com"], ipv4s: [], ipv6s: [], prices: [], hex_colors: ["201", "dea", "eac", "519", "236", "272", "341"], credit_cards: [], visas: [], mastercards: [], btc_addresses: [], street_addresses: [], zip_codes: [], po_boxs: [], ssns: [], md5s: [], sha1s: [], sha2s: [], guids: [], isbn13s: [], isbn10s: [], mac_addresses: [], ibans: [], gitrepos: [] } */
println!("{:?}", parsed.dates);
//prints ["Jan 9th 2012"]
println!("{:?}", parsed.times);
//prints ["5:00PM", "4:00"]
println!("{:?}",parsed.phones);
//prints ["(012)-345-6789"]
println!("{:?}",parsed.links);
//prints ["www.linkedin.com"]
println!("{:?}",parsed.emails);
//prints ["associative@mail.com"]
```   

Alternatively, you can generate a single CommonRegex instance and use it to parse multiple segments of text.
```
println!("{:?}",commonregex::times("When are you free? Do you want to meet up for coffee at 4:00?"));
//prints ["4:00"]
println!("{:?}",commonregex::prices("They said the price was $5,000.90, actually it is $3,900.5. It\'s $1100.4 less, can you imagine this?"));
//prints ["$5,000.90", "$3,900.5", "$110"]
println!("{:?}",commonregex::ipv6s("The IPv6 address for localhost is 0:0:0:0:0:0:0:1, or alternatively, ::1."));
//prints ["0:0:0:0:0:0:0:1", "::1"]
```

CommonRegex Ports
=================
There are CommonRegex ports for other languages, see [here](https://github.com/madisonmay/CommonRegex/#commonregex-ports "CommonRegex ports")
