use clipboard_win::{get_clipboard_string, set_clipboard_string};
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

fn get_clipboard_content(replace_crlf: bool) -> Result<String, String> {
    match get_clipboard_string() {
        Ok(content) => {
            if replace_crlf {
                Ok(content.replace("\r\n", "\n"))
            } else {
                Ok(content)
            }
        }
        Err(e) => {
            // FIXME: Consider if all errors should be silenced.
            // This is the legacy behavior, where we return empty string for most errors.
            if e.to_string().contains("Clipboard does not contain text") {
                Ok(String::new())
            } else {
                eprintln!("Failed to get clipboard content: {}", e);
                Ok(String::new())
            }
        }
    }
}

fn set_clipboard_content(content: &str, replace_lf: bool) -> Result<(), String> {
    let processed_content;
    let content_to_set = if replace_lf {
        // FIXME: This is not a robust way to handle line endings.
        // A simple replacement might not cover all cases, e.g., lone \r.
        processed_content = content.replace("\n", "\r\n");
        &processed_content
    } else {
        content
    };
    set_clipboard_string(content_to_set).map_err(|e| e.to_string())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_o {
        let content = get_clipboard_content(args.flag_lf).unwrap();
        print!("{}", content);
    } else if args.flag_i {
        let mut stdin = io::stdin();
        let mut content = String::new();
        stdin.read_to_string(&mut content).unwrap();
        set_clipboard_content(&content, args.flag_crlf).unwrap();
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
    set_clipboard_content(v, false).unwrap();
    assert_eq!(get_clipboard_content(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    let v = "Hello\rfrom\rwin32yank";
    set_clipboard_content(v, false).unwrap();
    assert_eq!(get_clipboard_content(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    let v = "Hello\r\nfrom\r\nwin32yank";
    set_clipboard_content(v, false).unwrap();
    assert_eq!(get_clipboard_content(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    let v = "\r\nfrom\r\nwin32yank\r\n\n...\\r\n";
    set_clipboard_content(v, false).unwrap();
    assert_eq!(get_clipboard_content(false).unwrap(), v);
    sleep(Duration::from_millis(sleep_time));

    set_clipboard_content("", true).unwrap();
    assert_eq!(get_clipboard_content(false).unwrap(), "");
    sleep(Duration::from_millis(sleep_time));

    set_clipboard_content("\n", true).unwrap();
    assert_eq!(get_clipboard_content(false).unwrap(), "\r\n");
    sleep(Duration::from_millis(sleep_time));

    set_clipboard_content("\r\n", true).unwrap();
    assert_eq!(get_clipboard_content(false).unwrap(), "\r\n");
    sleep(Duration::from_millis(sleep_time));

    let v = "\r\nfrom\r\nwin32yank\r\n\n...\\r\n";
    set_clipboard_content(v, true).unwrap();
    assert_eq!(
        get_clipboard_content(false).unwrap(),
        "\r\nfrom\r\nwin32yank\r\n\r\n...\\r\r\n"
    );
}
