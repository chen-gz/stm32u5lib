//
//
//
// /// USB PHY type
// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum PhyType {
//     /// Internal Full-Speed PHY
//     ///
//     /// Available on most High-Speed peripherals.
//     InternalFullSpeed,
//     /// Internal High-Speed PHY
//     ///
//     /// Available on a few STM32 chips.
//     InternalHighSpeed,
//     /// External ULPI High-Speed PHY
//     ExternalHighSpeed,
// }
//
// impl PhyType {
//     /// Get whether this PHY is any of the internal types.
//     pub fn internal(&self) -> bool {
//         match self {
//             PhyType::InternalFullSpeed | PhyType::InternalHighSpeed => true,
//             PhyType::ExternalHighSpeed => false,
//         }
//     }
//
//     /// Get whether this PHY is any of the high-speed types.
//     pub fn high_speed(&self) -> bool {
//         match self {
//             PhyType::InternalFullSpeed => false,
//             PhyType::ExternalHighSpeed | PhyType::InternalHighSpeed => true,
//         }
//     }
//
//     pub(crate) fn to_dspd(&self) -> stm32_metapac::otg::vals::Dspd {
//         match self {
//             PhyType::InternalFullSpeed => stm32_metapac::otg::vals::Dspd::FULL_SPEED_INTERNAL,
//             PhyType::InternalHighSpeed => stm32_metapac::otg::vals::Dspd::HIGH_SPEED,
//             PhyType::ExternalHighSpeed => stm32_metapac::otg::vals::Dspd::HIGH_SPEED,
//         }
//     }
// }
//
//
