use serde::{Deserialize, Serialize};

pub mod nodejs;
pub mod rust;
pub mod zig;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct FullConfig<'a> {
    #[serde(borrow)]
    nodejs: nodejs::NodejsConfig<'a>,
    #[serde(borrow)]
    rust: rust::RustConfig<'a>,
    #[serde(borrow)]
    zig: zig::ZigConfig<'a>,
}
