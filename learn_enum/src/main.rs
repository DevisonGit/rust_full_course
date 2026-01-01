fn main() {
    enum IpAddrKind {
        V4,
        V6
    }

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {
    }


    enum IpAddrKind {
        V4,
        V6
    }

    enum IpAddr {
        kind: IpAddrKind,
        address: String
    }

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.0"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };



}
