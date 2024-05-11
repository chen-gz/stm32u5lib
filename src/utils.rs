#![feature(panic_info_message)]

use core::panic::PanicInfo;

#[cfg(feature = "utils")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    defmt::info!("panic");
    defmt::error!(
        "Location file name: {:?}, line: {:?}, col: {:?}",
        _info.location().unwrap().file(),
        _info.location().unwrap().line(),
        _info.location().unwrap().column()
    );
    if let Some(args) = _info.message() {
        defmt::error!("Panic message: {:?}", args.as_str());
    }
    loop {}
}
