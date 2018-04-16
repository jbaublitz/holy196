use neli::{MemWrite,Nl};
use neli::err::NlError;
use neli::ffi::{CtrlAttr,CtrlAttrMcastGrp,CtrlCmd,GenlId,NlFamily,NlmF};
use neli::genlhdr::GenlHdr;
use neli::nlhdr::{NlAttrHdr,NlHdr};
use neli::socket::NlSocket;

pub fn resolve_gtk_rekey_multicast_group() -> Result<u32, NlError> {
    let mut socket = NlSocket::<GenlId, GenlHdr>::connect(NlFamily::Generic, None, None)?;
    let attrs = vec![NlAttrHdr::new_str_payload(None, CtrlAttr::FamilyName, "nl80211")?];
    let genlhdr = GenlHdr::new(CtrlCmd::Getfamily, 2, attrs)?;
    let nlhdr = NlHdr::new(None, GenlId::Ctrl, vec![NlmF::Request, NlmF::Ack], None, None, genlhdr);
    let mut mem_req = MemWrite::new_vec(Some(nlhdr.asize()));
    nlhdr.serialize(&mut mem_req)?;
    socket.send(mem_req.into(), 0)?;

    let mem_resp = MemWrite::new_vec(Some(4096));
    let mut mem_resp_recv = socket.recv(mem_resp, 0)?;
    let nlhdr = NlHdr::<GenlId, GenlHdr>::deserialize(&mut mem_resp_recv)?;
    let mut handle = nlhdr.nl_payload.get_attr_handle::<CtrlAttr>();
    let mut mcast_groups = handle.get_nested_attributes::<u16>(CtrlAttr::McastGroups)?;
    let mut id = None;
    for attr_num in 1..7 {
        let attribute = mcast_groups.parse_nested_attributes()?.get_attribute(attr_num)
                .ok_or(NlError::new("Index out of range"))?.nla_len;
        let string = mcast_groups.get_nested_attributes(attr_num)?
            .get_payload_with::<String>(CtrlAttrMcastGrp::Name, Some(attribute as usize))?;
        if string == "mlme".to_string() {
            let mut mcast_group = mcast_groups.get_nested_attributes::<CtrlAttrMcastGrp>(attr_num)?;
            id = mcast_group.get_payload_as::<u32>(CtrlAttrMcastGrp::Id).ok();
        }
    }

    id.ok_or(NlError::new("Failed to resolve multicast group ID"))
}
