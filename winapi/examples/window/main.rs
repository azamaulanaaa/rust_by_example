use windows;

fn main() {
    unsafe {
        let hinstance = windows::Win32::System::LibraryLoader::GetModuleHandleA(None).unwrap();
        debug_assert!(hinstance.0 != 0);

        let wc = windows::Win32::UI::WindowsAndMessaging::WNDCLASSA {
            hCursor: windows::Win32::UI::WindowsAndMessaging::LoadCursorW(
                None,
                windows::Win32::UI::WindowsAndMessaging::IDC_ARROW,
            )
            .unwrap(),
            hInstance: hinstance,
            lpszClassName: windows::s!("main"),
            style: windows::Win32::UI::WindowsAndMessaging::CS_HREDRAW
                | windows::Win32::UI::WindowsAndMessaging::CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = windows::Win32::UI::WindowsAndMessaging::RegisterClassA(&wc);
        debug_assert!(atom != 0);

        windows::Win32::UI::WindowsAndMessaging::CreateWindowExA(
            windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE::default(),
            wc.lpszClassName,
            windows::s!("Main"),
            windows::Win32::UI::WindowsAndMessaging::WS_OVERLAPPEDWINDOW
                | windows::Win32::UI::WindowsAndMessaging::WS_VISIBLE,
            windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT,
            windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT,
            windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT,
            windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT,
            None,
            None,
            hinstance,
            None,
        );

        let mut message = windows::Win32::UI::WindowsAndMessaging::MSG::default();
        while windows::Win32::UI::WindowsAndMessaging::GetMessageA(
            &mut message,
            windows::Win32::Foundation::HWND(0),
            0,
            0,
        )
        .into()
        {
            windows::Win32::UI::WindowsAndMessaging::DispatchMessageA(&message);
        }
    }
}

extern "system" fn wndproc(
    hwnd: windows::Win32::Foundation::HWND,
    message: u32,
    wparam: windows::Win32::Foundation::WPARAM,
    lparam: windows::Win32::Foundation::LPARAM,
) -> windows::Win32::Foundation::LRESULT {
    unsafe {
        match message {
            windows::Win32::UI::WindowsAndMessaging::WM_PAINT => {
                windows::Win32::Foundation::LRESULT(0)
            }
            windows::Win32::UI::WindowsAndMessaging::WM_DESTROY => {
                windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
                windows::Win32::Foundation::LRESULT(0)
            }
            _ => windows::Win32::UI::WindowsAndMessaging::DefWindowProcA(
                hwnd, message, wparam, lparam,
            ),
        }
    }
}
