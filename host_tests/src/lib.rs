#![allow(unused)]
#[path = "../../src/time_math.rs"]
mod time_math;

use time_math::compute_walltimer;

#[test]
fn test_compute_walltimer_normal() {
    let high = 0;
    let low = 1000;
    let ue = false;
    let resolution_ns = 1000; // 1us

    let ticks = compute_walltimer(high, low, ue, resolution_ns);
    // 0 * 65536 + 1000 = 1000 ticks
    // 1000 ticks * 1000 ns / 1000 = 1000 us
    assert_eq!(ticks, 1000);
}

#[test]
fn test_compute_walltimer_overflow_pending() {
    // Case: Overflow happened (ue=true), cnt wrapped (low is small), ISR not run (high not inc)
    let high = 0;
    let low = 100; // wrapped
    let ue = true;
    let resolution_ns = 1000; // 1us

    let ticks = compute_walltimer(high, low, ue, resolution_ns);
    // high should be treated as 1.
    // (1 * 65536 + 100) * 1000 / 1000 = 65636
    assert_eq!(ticks, 65536 + 100);
}

#[test]
fn test_compute_walltimer_high_values() {
    let high = 10;
    let low = 50000;
    let ue = false;
    let resolution_ns = 1000;

    let ticks = compute_walltimer(high, low, ue, resolution_ns);
    // 10 * 65536 + 50000 = 655360 + 50000 = 705360
    assert_eq!(ticks, 705360);
}

#[test]
fn test_compute_walltimer_resolution() {
    let high = 0;
    let low = 1000;
    let ue = false;
    let resolution_ns = 30517; // ~32768Hz (1/32768 * 1e9)

    let ticks = compute_walltimer(high, low, ue, resolution_ns);
    // 1000 * 30517 / 1000 = 30517
    assert_eq!(ticks, 30517);
}

#[test]
fn test_ue_ignored_if_low_high() {
    // Current implementation ignores ue if low >= 0x8000
    // This assumes ISR latency is < half period.
    let high = 0;
    let low = 0x8000;
    let ue = true;
    let resolution_ns = 1000;

    let ticks = compute_walltimer(high, low, ue, resolution_ns);
    // 0 + 0x8000 = 32768
    assert_eq!(ticks, 32768);
}
