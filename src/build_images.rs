pub const CLONE_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/clone.tar"));
pub const FETCH_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fetch.tar"));
pub const BUILD_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/build.tar"));
pub const BENCH_END_TO_END: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/bench-end-to-end.tar"));