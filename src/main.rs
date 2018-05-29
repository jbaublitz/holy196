extern crate neli;
extern crate tokio;

mod netlink;

pub fn main() {
    let gtk = netlink::NetlinkGtk::new().unwrap();
    gtk.gtk_loop().unwrap();
}
