A clipboard tool for Windows.
This requires Windows 7 SP1 or above (as supported by rust).
This support WSL2 cross-platform clipboard sharing.(Can build by `make` too)

Get the clipboard.

    win32yank -o

Set the clipboard

    echo "hello brave new world!" | win32yank -i

