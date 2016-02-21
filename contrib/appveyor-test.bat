if "%APPVEYOR_REPO_TAG%" == "true" (
	echo "Release Build"
	cargo test --release
) else (
	cargo test
)
