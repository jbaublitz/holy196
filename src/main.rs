extern crate neli;

mod netlink;

pub fn main() {
    println!("{}", netlink::resolve_gtk_rekey_multicast_group().unwrap());
}
