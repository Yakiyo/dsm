curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall lsd cargo-edit bat -y

echo alias bat="bat --pager=never" >> ~/.bashrc

# Install oh-my-posh
curl -s https://ohmyposh.dev/install.sh | bash -s

echo eval "$(oh-my-posh init bash)" >> ~/.bashrc