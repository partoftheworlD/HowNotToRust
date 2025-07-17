use std::sync::Mutex;

fn main() {
    let x = Mutex::new(0);
    {
        let mut z = x.lock().unwrap();
        *z = 1337;
    }

    let mutex_value = unsafe { *((&raw const x).cast::<i32>()).byte_add(0x4) as *const i32 } as i32;

    assert_eq!(mutex_value, *x.lock().unwrap());
}
