// mod bus;
// mod endpoint;
// mod control_pipe;
pub mod control_pipe;
pub mod endpoint;
pub mod global_states;
mod interrupt;
mod phy_type;
pub mod power;

// let res = process_setup_packet(SETUP_DATA);

use aligned::Aligned;
// use crate::otg_hs::descriptor::*;
use crate::otg_hs::endpoint::{Endpoint, EpType, MaxPacketSize};


pub struct EndpointGG;

pub async fn cdc_acm_ep2_read() -> Result<(Aligned<aligned::A4, [u8; 64]>, usize), ()> {
    let ep2_out = Endpoint::new(
        crate::otg_hs::endpoint::Direction::Out,
        2,
        EpType::Bulk,
        MaxPacketSize::Size64,
        0,
    )
    .unwrap();
    let mut buf: Aligned<aligned::A4, [u8; 64]> = Aligned([0u8; 64]);
    match ep2_out.read(&mut buf[0..64]).await {
        Ok(len) => {
            defmt::info!("cdc_acm_ep2_read, {:?}, len={}", &buf[0..len], len);
            return Ok((buf, len));
        }
        Err(_e) => {
            // defmt::info!("cdc_acm_ep2_read, {:?}", e);
            // return (buf, 0);
            return Err(());
        }
    }
}
pub async fn cdc_acm_ep2_write(buf: &[u8]) {
    let ep2_in = Endpoint::new(
        crate::otg_hs::endpoint::Direction::In,
        2,
        EpType::Bulk,
        MaxPacketSize::Size64,
        0,
    )
    .unwrap();
    let _ = ep2_in.write(buf).await;
}
