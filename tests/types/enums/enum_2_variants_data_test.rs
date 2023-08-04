enum IpAddr {
    V4(String), // enum variant and its data
    V6(String), // enum variant and its data
}

impl IpAddr {
    fn to_string(&self) -> &str {
        match self {
            IpAddr::V4(_) => "V4",
            IpAddr::V6(_) => "V6",
        }
    }
}

#[test]
fn enum_variants_test() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    assert_eq!("V4", home.to_string(), "enum value");

    let addr = match loopback {
        IpAddr::V4(addr) => addr,
        IpAddr::V6(addr) => addr,
    };

    assert_eq!("::1", addr, "enum variant's data");
}
