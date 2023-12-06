use embedded_sdmmc::{Block, BlockCount, BlockDevice, BlockIdx, Error, Mode, Timestamp};

use crate::sdmmc::{SdError, SdInstance};

impl BlockDevice for SdInstance {
    type Error = SdError;
    fn read(
        &self,
        blocks: &mut [Block],
        start_block_idx: BlockIdx,
        reason: &str,
    ) -> Result<(), Self::Error> {
        let len = blocks.len() * 256;
        let data = unsafe { core::slice::from_raw_parts_mut(blocks.as_mut_ptr() as *mut u8, len) };
        if (blocks.len() == 1) {
            return self.read_single_block(data, start_block_idx.0);
        }
        return self.read_multiple_blocks(data, start_block_idx.0, blocks.len() as u32);
    }
    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        let len = blocks.len() * 256;
        let data = unsafe { core::slice::from_raw_parts(blocks.as_ptr() as *const u8, len) };
        let mut ok = false;
        let mut err = SdError::AckFail;
        if  blocks.len() == 1{
            return self.write_single_block(data, start_block_idx.0);
        }
        for i in 0..10{
            match self.write_multiple_blocks(data, start_block_idx.0, blocks.len() as u32){
                Ok(_) => {
                    ok = true;
                    break;
                },
                Err(e) => {err = e;},
            }
        }
        if ok{
            Ok(())
        }else{
            Err(err)
        }

    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(BlockCount(self.block_count()))
    }
}
