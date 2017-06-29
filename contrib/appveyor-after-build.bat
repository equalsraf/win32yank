if "%APPVEYOR_REPO_TAG%" == "true" (
	echo "Packaging Release zip"
	if "%PLATFORM%" == "*x86_64-pc-windows-msvc.exe" (
        7z a win32yank-x64.zip LICENSE README.md %APPVEYOR_BUILD_FOLDER%\target\release\win32yank.exe
	)
	if "%PLATFORM%" == "*i686-pc-windows-msvc.exe" (
        7z a win32yank-x86.zip LICENSE README.md %APPVEYOR_BUILD_FOLDER%\target\release\win32yank.exe
	)
)
