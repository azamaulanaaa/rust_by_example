use windows;

fn main() {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::MessageBoxA(
            None,
            windows::s!("Hello, world!"),
            windows::s!("Horray!!"),
            windows::Win32::UI::WindowsAndMessaging::MB_OK,
        );
    }
}
