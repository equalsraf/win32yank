use clipboard_win::{formats, SysResult};
use docopt::Docopt;
use serde::Deserialize;
use std::io;
use std::io::Read;

const USAGE: &str = "
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

#[derive(Debug, Deserialize)]
struct Args {
    flag_o: bool,
    flag_i: bool,
    flag_lf: bool,
    flag_crlf: bool,
}

fn get_clipboard(replace_crlf: bool) -> SysResult<String> {
    let content: SysResult<String> = clipboard_win::get_clipboard(formats::Unicode);
    if replace_crlf {
        content.map(|data| data.replace("\r\n", "\n"))
    } else {
        content
    }
}

fn set_clipboard(content: &str, replace_lf: bool) -> SysResult<()> {
    if replace_lf {
        let chunks = content.split("\r\n").map(|item| item.replace("\n", "\r\n"));
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
        clipboard_win::set_clipboard(formats::Unicode, &out)
    } else {
        clipboard_win::set_clipboard(formats::Unicode, content)
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_o {
        let content: String = get_clipboard(args.flag_crlf).unwrap();
        print!("{}", content);
    } else if args.flag_i {
        let mut stdin = io::stdin();
        let mut content = String::new();
        stdin.read_to_string(&mut content).unwrap();
        set_clipboard(&content, args.flag_lf).unwrap();
    }
}

#[test]
fn test() {
    // Windows dislikes if we lock the clipboard too long sleep for bit
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
    assert_eq!(
        get_clipboard(false).unwrap(),
        "\r\nfrom\r\nwin32yank\r\n\r\n...\\r\r\n"
    );
}
