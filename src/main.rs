extern crate clipboard_win;
extern crate docopt;
extern crate rustc_serialize;
extern crate winapi;
extern crate user32;
extern crate kernel32;

use clipboard_win::WindowsError;
use clipboard_win::wrapper::{get_last_error, open_clipboard, close_clipboard};
use clipboard_win::clipboard_formats::CF_UNICODETEXT;
use docopt::Docopt;
use kernel32::{GlobalLock, GlobalUnlock};
use winapi::winnt::HANDLE;
use user32::GetClipboardData;
use std::io;
use std::io::Read;

const USAGE: &'static str = "
win32yank

Usage:
    win32yank -o [--lf]
    win32yank -i [--crlf]

Options:
    -o          Print clipboard contents to stdout
    -i          Set clipboard from stdin
    --lf        Replace CRLF with LF before printing to stdout
    --crlf      Replace lone LF bytes with CRLF before setting the clipboard
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_o: bool,
    flag_i: bool,
    flag_lf: bool,
    flag_crlf: bool,
}

fn from_wide_ptr(ptr: *const u16) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    if ptr.is_null() {
        return String::new();
    }

    unsafe {
        assert!(!ptr.is_null());
        let len = (0..std::isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap();
        let slice = std::slice::from_raw_parts(ptr, len);
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}

fn get_clipboard(replace_crlf: bool) -> Result<String, WindowsError> {
    let result: Result<String, WindowsError>;
    try!(open_clipboard());
    unsafe {
        let text_handler: HANDLE = GetClipboardData(CF_UNICODETEXT as u32);

        if text_handler.is_null() {
            result = Err(get_last_error());
        } else {
            let text_p = GlobalLock(text_handler) as *const u16;
            result = Ok(from_wide_ptr(text_p));
            GlobalUnlock(text_handler);
        }
    }
    try!(close_clipboard());

    if replace_crlf {
        result.map(|data| data.replace("\r\n", "\n"))
    } else {
        result
    }
}

fn set_clipboard(content: &str, replace_lf: bool) -> Result<(), WindowsError> {
    // clipboard_win::wrapper::set_clipboard uses CF_UNICODETEXT,
    // which is what we want

    if replace_lf {
        let chunks = content.split("\r\n")
            .map(|item| item.replace("\n", "\r\n"));
        let mut first = true;
        let mut out = String::with_capacity(content.len());
        for chunk in chunks {
            if first {
                first = false;
            } else {
                out.push_str("\r\n");
            }
            out.push_str(&chunk);
        }
        clipboard_win::set_clipboard(&out)
    } else {
        clipboard_win::set_clipboard(content)
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_o {
        let content = get_clipboard(args.flag_lf).unwrap();
        println!("{}", content);
    } else if args.flag_i {
        let mut stdin = io::stdin();
        let mut content = String::new();
        stdin.read_to_string(&mut content).unwrap();
        set_clipboard(&content, args.flag_crlf).unwrap();
    }
}

#[test]
fn test() {
    // Windows dislikes if we lock the clipboard too long
    // sleep for bit
    use std::thread::sleep;
    use std::time::Duration;
    let sleep_time = 300;

    let v = "Hello\nfrom\nwin32yank";
    set_clipboard(v, false).unwrap();
    assert_eq!(get_clipboard(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    let v = "Hello\rfrom\rwin32yank";
    set_clipboard(v, false).unwrap();
    assert_eq!(get_clipboard(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    let v = "Hello\r\nfrom\r\nwin32yank";
    set_clipboard(v, false).unwrap();
    assert_eq!(get_clipboard(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    let v = "\r\nfrom\r\nwin32yank\r\n\n...\\r\n";
    set_clipboard(v, false).unwrap();
    assert_eq!(get_clipboard(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    //
    // set_clipboard(true)
    set_clipboard("", true).unwrap();
    assert_eq!(get_clipboard(false).unwrap(), "");
    sleep(Duration::from_millis(sleep_time));

    set_clipboard("\n", true).unwrap();
    assert_eq!(get_clipboard(false).unwrap(), "\r\n");
    sleep(Duration::from_millis(sleep_time));

    set_clipboard("\r\n", true).unwrap();
    assert_eq!(get_clipboard(false).unwrap(), "\r\n");
    sleep(Duration::from_millis(sleep_time));

    let v = "\r\nfrom\r\nwin32yank\r\n\n...\\r\n";
    set_clipboard(v, true).unwrap();
    assert_eq!(get_clipboard(false).unwrap(),
               "\r\nfrom\r\nwin32yank\r\n\r\n...\\r\r\n");
}
