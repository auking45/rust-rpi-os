//use cortex_a::asm;

#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            asm!(
                "wfe",
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}
