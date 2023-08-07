struct Ipv4Addr {
    addr1: u8,
    addr2: u8,
    addr3: u8,
    addr4: u8,
}

struct Ipv6Addr {
    addr: String,
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    fn to_string(&self) -> String {
        match self {
            IpAddr::V4(v4) => format!("{}.{}.{}.{}", v4.addr1, v4.addr2, v4.addr3, v4.addr4),
            IpAddr::V6(v6) => v6.addr.clone(),
        }
    }
}

#[test]
fn enum_variants_with_struct_data_test() {
    let home = IpAddr::V4(Ipv4Addr {
        addr1: 127,
        addr2: 0,
        addr3: 0,
        addr4: 1,
    });
    let loopback = IpAddr::V6(Ipv6Addr {
        addr: String::from("::1"),
    });

    assert_eq!("127.0.0.1", home.to_string(), "enum to_string");
    assert_eq!("::1", loopback.to_string(), "enum to_string");
}
