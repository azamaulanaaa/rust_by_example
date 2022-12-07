use std::ptr::null_mut;
use winapi::um::winuser;

fn main() {
    let title = b"Horray!!!\0";
    let body = b"Hello, world!\0";

    unsafe {
        winuser::MessageBoxA(
            null_mut(),
            body.as_ptr().cast(),
            title.as_ptr().cast(),
            winuser::MB_OK,
        );
    }
}
