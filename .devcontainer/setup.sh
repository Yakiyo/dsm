## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  bash \
  vim \
  build-essential \
  openssl \
  unzip

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy 
rustup component add clippy --toolchain nightly

cargo install cargo-expand
cargo install cargo-edit
cargo install --locked bat
cargo install lsd

echo alias bat="bat --pager=never" >> ~/.bashrc
# Install oh-my-posh
curl -s https://ohmyposh.dev/install.sh | bash -s

echo eval "$(oh-my-posh init bash)" >> ~/.bashrc
exec bash