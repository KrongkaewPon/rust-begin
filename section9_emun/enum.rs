#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum IpAddrKind{
    v4,v6
}

fn main() {
    let four=IpAddrKind::v4;
    let six=IpAddrKind::v6;
    route(four);
    route(six);
}

fn route(ip:IpAddrKind) {
    println!("{:?}", ip);
}