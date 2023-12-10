use embedded_sdmmc::{Block, BlockCount, BlockDevice, BlockIdx};

use crate::sdmmc::{SdError, SdInstance};

impl BlockDevice for SdInstance {
    type Error = SdError;
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        // defmt::info!("read start_block_idx: {:?} reason: {}", start_block_idx, reason);
        let len = blocks.len() * 256;
        let data = unsafe { core::slice::from_raw_parts_mut(blocks.as_mut_ptr() as *mut u8, len) };
        if blocks.len() == 1 {
            return self.read_single_block_retry(data, start_block_idx.0, 3);
        } else {
            defmt::panic!("read multiple blocks")
        }
    }
    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        // defmt::info!("write start_block_idx: {:?}", start_block_idx);
        let len = blocks.len() * 256;
        let data = unsafe { core::slice::from_raw_parts(blocks.as_ptr() as *const u8, len) };
        if blocks.len() == 1 {
            return self.write_single_block(data, start_block_idx.0);
        } else {
            defmt::panic!("write multiple blocks")
        }
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(BlockCount(self.block_count()))
    }
}
