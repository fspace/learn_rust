//  相当于nodejs中的index文件

// 声明子模块
mod destruct;
mod drop;
mod ops;
mod traits;
mod unsafes;

pub mod funcs;

//  导出
pub use self::destruct::*;
pub use self::drop::*;
pub use self::ops::*;
pub use self::traits::*;
pub use self::unsafes::*;
