pub struct NoSyncWrap<T>(T);

impl<T> NoSyncWrap<T> {
    pub fn new(inner: T) -> Self {
        NoSyncWrap(inner)
    }
    pub fn get(&self) -> Option<&T> {
        unsafe {
            let a = &self.0 as *const T as *mut T;
            a.as_ref()
        }
    }
    pub fn get_mut(&self) -> Option<&mut T> {
        unsafe {
            let a = &self.0 as *const T as *mut T;
            a.as_mut()
        }
    }
    pub fn get_raw(&self) -> Option<T> {
        unsafe {
            let a = &self.0 as *const T as *mut T;
            let a1 = a.read();
            Some(a1)
        }
    }
}

unsafe impl<T> Send for NoSyncWrap<T> {}
unsafe impl<T> Sync for NoSyncWrap<T> {}
