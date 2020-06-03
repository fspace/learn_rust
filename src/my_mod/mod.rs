//  相当于nodejs中的index文件

// 声明子模块
mod add;
mod sub;

pub mod misc;
pub mod patterns;

// 重新导出
// 等价 ==> pub use self::{add:add, sub::sub};
pub use add::add;
pub use sub::sub;
// pub use self::{add::*, sub::*};  可以使用通配符 全部导出

const DEBUG: bool = true;
