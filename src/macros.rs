#[macro_export]
macro_rules! usize_buf {
    () => (
        std::vec::Vec::<u8>::with_capacity((usize::BITS / u8::BITS) as usize)
    );
}