/// 进程对象
pub struct Process {
    base: KObjectBase,
    inner: Mutex<ProcessInner>,
}

impl_kobject!(Process);

struct ProcessInner {
    handles: BTreeMap<HandleValue, Handle>,
}

pub type HandleValue = u32;

impl Process {
    /// 创建一个新的进程对象
    pub fn new() -> Arc<Self> {
        Arc::new(Process {
            base: KObjectBase::default(),
            inner: Mutex::new(ProcessInner {
                handles: BTreeMap::default(),
            }),
        })
    }
}

impl Process {
    /// 添加一个新的对象句柄
    pub fn add_handle(&self, handle: Handle) -> HandleValue {
        let mut inner = self.inner.lock();
        let value = (0 as HandleValue..)
            .find(|idx| !inner.handles.contains_key(idx))
            .unwrap();
        inner.handles.insert(value, handle);
        value
    }

    /// 删除一个对象句柄
    pub fn remove_handle(&self, handle_value: HandleValue) {
        self.inner.lock().handles.remove(&handle_value);
    }
}

impl Process {
    /// 根据句柄值查找内核对象，并检查权限
    pub fn get_object_with_rights<T: KernelObject>(
        &self,
        handle_value: HandleValue,
        desired_rights: Rights,
    ) -> ZxResult<Arc<T>> {
        let handle = self
            .inner
            .lock()
            .handles
            .get(&handle_value)
            .ok_or(ZxError::BAD_HANDLE)?
            .clone();
        // check type before rights
        let object = handle
            .object
            .downcast_arc::<T>()
            .map_err(|_| ZxError::WRONG_TYPE)?;
        if !handle.rights.contains(desired_rights) {
            return Err(ZxError::ACCESS_DENIED);
        }
        Ok(object)
    }
}