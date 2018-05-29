use neli::err::NlError;
use neli::ffi::{CtrlCmd,GenlId,NlFamily};
use neli::genlhdr::GenlHdr;
use neli::socket::NlSocket;
use tokio;
use tokio::prelude::stream::Stream;

pub struct NetlinkGtk {
    #[allow(dead_code)]
    gtk: Vec<u8>,
    listen_sock: NlSocket<GenlId, GenlHdr<CtrlCmd>>,
}

impl NetlinkGtk {
    pub fn new() -> Result<Self, NlError> {
        let listen_sock = {
            let mut resolve_sock = NlSocket::<GenlId, GenlHdr<CtrlCmd>>::new_genl()?;
            let mcast_id = resolve_sock.resolve_nl_mcast_group("nl80211", "mlme")?;
            let sock = NlSocket::connect(NlFamily::Generic, None, vec![mcast_id])?;
            sock
        };
        Ok(NetlinkGtk { gtk: Vec::new(), listen_sock })
    }

    pub fn gtk_loop(self) -> Result<(), NlError> {
        tokio::run(self.listen_sock.for_each(|_item| {
            println!("WPA negotiation");
            Ok(())
        }));
        Ok(())
    }
}
