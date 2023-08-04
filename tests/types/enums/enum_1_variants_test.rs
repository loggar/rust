use lib_root::lib_types::get_type_of;

// enum type
enum IpAddrKind {
    V4, // variant
    V6, // variant
}

impl IpAddrKind {
    fn to_string(&self) -> &str {
        match self {
            IpAddrKind::V4 => "V4",
            IpAddrKind::V6 => "V6",
        }
    }
}

fn route(ip_type: IpAddrKind) -> String {
    return String::from(ip_type.to_string());
}

#[test]
fn enum_variants_test() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    assert_eq!(
        "_t_types_tests::types::enums::enum_1_variants_test::IpAddrKind",
        get_type_of(&four),
        "get_type_of check"
    );

    let r1 = route(IpAddrKind::V4);
    let r2 = route(six);

    assert_eq!("V4", r1, "enum value");
    assert_eq!("V6", r2, "enum value");
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[test]
fn enum_in_struct_test() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    assert_eq!("V4", home.kind.to_string(), "enum value");
    assert_eq!("::1", loopback.address, "enum value");
}
