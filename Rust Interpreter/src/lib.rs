pub mod parser;

#[cfg(feature = "wasm")]
mod wasm_api;


#[macro_export]
macro_rules! ghidra_marker {
    ($reg:literal) => {
        unsafe {
            // let f = concat!("mov ", $reg, ",", $reg);
            std::arch::asm!(concat!("mov ", $reg, ",", $reg));
            // std::arch::asm!("mov {0}, {0}", inout("eax") _a);
        };
    };
}
