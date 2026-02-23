/// Pure logic for WallTimer time calculation.
pub fn compute_walltimer(high: u128, low: u32, ue: bool, resolution_ns: u64) -> u64 {
    let mut total_ticks = high * 0x10000 + (low as u128);
    if ue && low < 0x8000 {
        total_ticks = (high + 1) * 0x10000 + (low as u128);
    }
    ((total_ticks * (resolution_ns as u128)) / 1000) as u64
}
