
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
    // defmt::error!("{:?}", _info.message().unwrap());

    // if let Some(message) = _info.message() {
    //     defmt::error!("{}", message);
    // } else {
    //     defmt::error!("No message available.");
    // }
    defmt::error!("Panic message: ");
    if let Some(args) = _info.message() {
        // Attempt to format message into a stack-allocated buffer if environment permits
        // Otherwise, log a generic message
        defmt::error!("Panic message: {:?}", args.as_str());
    } else {
        defmt::error!("No panic message available.");
    }
    loop {}
}
