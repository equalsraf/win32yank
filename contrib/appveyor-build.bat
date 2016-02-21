if "%APPVEYOR_REPO_TAG%" == "true" (
	echo "Release Build"
	cargo build --release
) else (
	cargo build
)
