use core::cmp;

pub struct Timings {
    pub prescale: u8,
    pub scll: u8,
    pub sclh: u8,
    pub sdadel: u8,
    pub scldel: u8,
}

impl Timings {
    pub fn new(i2cclk: u32, frequency: u32) -> Self {
        let ratio = i2cclk / frequency;

        // For the standard-mode configuration method, we must have a ratio of 4
        // or higher
        assert!(ratio >= 4, "The I2C PCLK must be at least 4 times the bus frequency!");

        let (presc_reg, scll, sclh, sdadel, scldel) = if frequency > 100_000 {
            // Fast-mode (Fm) or Fast-mode Plus (Fm+)
            // here we pick SCLL + 1 = 2 * (SCLH + 1)

            // Prescaler, 384 ticks for sclh/scll. Round up then subtract 1
            let presc_reg = ((ratio - 1) / 384) as u8;
            // ratio < 1200 by pclk 120MHz max., therefore presc < 16

            // Actual precale value selected
            let presc = (presc_reg + 1) as u32;

            let sclh = ((ratio / presc) - 3) / 3;
            let scll = (2 * (sclh + 1)) - 1;

            let (sdadel, scldel) = if frequency > 400_000 {
                // Fast-mode Plus (Fm+)
                assert!(i2cclk >= 17_000_000); // See table in datsheet

                let sdadel = i2cclk / 8_000_000 / presc;
                let scldel = i2cclk / 4_000_000 / presc - 1;

                (sdadel, scldel)
            } else {
                // Fast-mode (Fm)
                assert!(i2cclk >= 8_000_000); // See table in datsheet

                let sdadel = i2cclk / 4_000_000 / presc;
                let scldel = i2cclk / 2_000_000 / presc - 1;

                (sdadel, scldel)
            };

            (presc_reg, scll as u8, sclh as u8, sdadel as u8, scldel as u8)
        } else {
            // Standard-mode (Sm)
            // here we pick SCLL = SCLH
            assert!(i2cclk >= 2_000_000); // See table in datsheet

            // Prescaler, 512 ticks for sclh/scll. Round up then
            // subtract 1
            let presc = (ratio - 1) / 512;
            let presc_reg = cmp::min(presc, 15) as u8;

            // Actual prescale value selected
            let presc = (presc_reg + 1) as u32;

            let sclh = ((ratio / presc) - 2) / 2;
            let scll = sclh;

            // Speed check
            assert!(sclh < 256, "The I2C PCLK is too fast for this bus frequency!");

            let sdadel = i2cclk / 2_000_000 / presc;
            let scldel = i2cclk / 500_000 / presc - 1;

            (presc_reg, scll as u8, sclh as u8, sdadel as u8, scldel as u8)
        };

        // Sanity check
        assert!(presc_reg < 16);

        // Keep values within reasonable limits for fast per_ck
        let sdadel = cmp::max(sdadel, 2);
        let scldel = cmp::max(scldel, 4);

        Self {
            prescale: presc_reg,
            scll,
            sclh,
            sdadel,
            scldel,
        }
    }
}
