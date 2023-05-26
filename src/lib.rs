use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;
pub use tokio_shared_rt_macro::test;

pub static RUNTIME: OnceCell<Runtime> = OnceCell::new();
