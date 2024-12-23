use crate::rpos_drv;

pub use rpos_drv::Error;
pub use rpos_drv::RposError;

pub type Result<T> = std::result::Result<T, Error>;
