if "%APPVEYOR_REPO_TAG%" == "true" (
	echo "Packaging Release zip"
        7z a win32yank.zip LICENSE README.md %APPVEYOR_BUILD_FOLDER%\target\release\win32yank.exe
)
