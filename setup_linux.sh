cargo build --release
sudo cp target/release/gm /usr/bin/gm
mkdir -p ~/bin
cp target/release/gm ~/bin/gm
if ! grep -q 'export PATH="$HOME/bin:$PATH"' ~/.bashrc; then
    echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
fi
source ~/.bashrc