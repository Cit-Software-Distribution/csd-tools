ARCH=$(uname -m)
OS=$(uname -s)
BIN_DIR="$HOME/.local/bin"

# echo "$OS $ARCH"

if [[ "$OS" == "Linux" && "$ARCH" == "x86_64" ]]; then
  echo "Downloading csd from CSD repository..."
  mkdir -p "$BIN_DIR"
  
  curl -sSfL "https://github.com/Cit-Software-Distribution/csd-tools/releases/latest/download/csd-x86_64-linux.tar.gz" | tar -xzf - -C "$BIN_DIR"
  
  chmod +x "$BIN_DIR/csd"

  echo "csd installed successfully in $BIN_DIR!"
  echo "make sure $BIN_DIR is in yout PATH."
else
  echo "Unsupported OS ($OS) or Architecture ($ARCH)"

fi
