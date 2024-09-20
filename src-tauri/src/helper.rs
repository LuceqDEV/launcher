pub unsafe fn ref_to_mut<T: ?Sized>(val: &T) -> &mut T {
    (val as *const T as *mut T).as_mut().unwrap_unchecked()
}