use alloc::string::String;
use core::fmt::Debug;
use downcast_rs::{impl_downcast, DowncastSync};

/// 内核对象公共接口
pub trait KernelObject: Send + Sync {
    /// 获取对象 ID
    fn id(&self) -> KoID;
    /// 获取对象类型名
    fn type_name(&self) -> &str;
    /// 获取对象名称
    fn name(&self) -> String;
    /// 设置对象名称
    fn set_name(&self, name: &str);
}

/// 对象 ID 类型
pub type KoID = u64;

pub trait KernelObject: DowncastSync + Debug {...}
impl_downcast!(sync KernelObject);