pub fn ip() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6{
        address: String::from("::1")
    };

}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6{
        address: String
    },
}

fn route() {

}