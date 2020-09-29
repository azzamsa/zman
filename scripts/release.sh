VERSION=`awk '/version = "[0-9]\.[0-9]\.[0-9]"/' Cargo.toml | awk -F\" '{print $2}'`

# init
cargo fmt

# GNU/Linux
cargo build --release
strip target/release/zman
mv target/release/zman target/release/zman-$VERSION

# Windows PC
cargo build --release --target=x86_64-pc-windows-gnu
mv target/x86_64-pc-windows-gnu/release/zman.exe target/x86_64-pc-windows-gnu/release/zman-$VERSION.exe

# don't forget!
echo "Check the changelog!" >> CHANGELOG.md
echo "Check the version!" >> Cargo.toml
