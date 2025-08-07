#!/usr/bin/env bash

# Colors and formatting
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color
BOLD=$(tput bold)
NORMAL=$(tput sgr0)

# Animation characters
SPINNER=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')

# Configuration
REPO_URL="https://github.com/Zer0C0d3r/Runtime.git"
INSTALL_DIR="/usr/local/bin"
TMP_DIR="/tmp/runtime-install-$(date +%s)"
BINARY_NAME="runtime"
VERSION="1.0.0"
INSTALL_DIR="/usr/local/bin"
TMP_DIR="/tmp/runtime-install"
BINARY_NAME="runtime"

# ASCII Art
print_banner() {
  echo -e "${MAGENTA}"
  cat << "EOF"
  _____             _   _                
 |  __ \           | | (_)               
 | |__) |   _ _ __ | |_ _ _ __ ___   ___ 
 |  _  / | | | '_ \| __| | '_ ` _ \ / _ \
 | | \ \ |_| | | | | |_| | | | | | |  __/
 |_|  \_\__,_|_| |_|\__|_|_| |_| |_|\___|                                     
EOF
  echo -e "${NC}"
}

# Animated spinner
spinner() {
  local pid=$!
  local i=0
  while kill -0 $pid 2>/dev/null; do
    echo -ne "\r${SPINNER[$i]} ${BLUE}Working...${NC}"
    i=$(( (i+1) % 10 ))
    sleep 0.1
  done
  echo -ne "\r${GREEN}✓${NC} Done!         \n"
}

# Check dependencies
check_dependencies() {
  echo -e "${YELLOW}🔍 Checking system dependencies...${NC}"
  local missing=()
  declare -A required_cmds=(
    ["cargo"]="Rust build system"
    ["git"]="Version control system"
  )
  
  for cmd in "${!required_cmds[@]}"; do
    if ! command -v "$cmd" &> /dev/null; then
      missing+=("$cmd (${required_cmds[$cmd]})")
    fi
  done
  
  if [ ${#missing[@]} -gt 0 ]; then
    echo -e "${RED}✗ Missing required dependencies:${NC}"
    printf " - %s\n" "${missing[@]}"
    echo -e "\n${CYAN}Please install missing packages and try again.${NC}"
    exit 1
  fi
  echo -e "${GREEN}✓ All dependencies found${NC}"
}

# Installation process
install_runtime() {
  echo -e "\n${BOLD}${CYAN}🚀 Starting Installation...${NC}${NORMAL}"
  
  # Clone repo
  echo -e "${YELLOW}📦 Cloning repository...${NC}"
  git clone $REPO_URL $TMP_DIR & spinner
  
  # Build project
  echo -e "\n${YELLOW}🔨 Building project...${NC}"
  (cd $TMP_DIR && cargo build --release) & spinner
  
  # Install binary
  echo -e "\n${YELLOW}📥 Installing binary...${NC}"
  sudo install -m 755 "${TMP_DIR}/target/release/${BINARY_NAME}" "$INSTALL_DIR" & spinner
  
  # Cleanup
  echo -e "${YELLOW}🧹 Cleaning up temporary files...${NC}"
  rm -rf "$TMP_DIR" & spinner

  # Verify installation
  echo -e "\n${YELLOW}🔎 Verifying installation...${NC}"
  if command -v $BINARY_NAME &> /dev/null; then
    echo -e "${GREEN}✓ ${BINARY_NAME} installed successfully in ${INSTALL_DIR}/${BINARY_NAME}${NC}"
    echo -e "\n${BOLD}${GREEN}🎉 Installation Complete!${NC}${NORMAL}"
    echo -e "Try it: ${CYAN}${BINARY_NAME} --format pretty --since${NC}\n"
  else
    echo -e "${RED}✗ Installation failed - binary not found in PATH${NC}"
    exit 1
  fi
}

# Uninstallation process
uninstall_runtime() {
  echo -e "\n${BOLD}${CYAN}🗑️  Starting Uninstallation...${NC}${NORMAL}"
  
  # Remove binary
  echo -e "${YELLOW}🚮 Removing binary...${NC}"
  sudo rm -f "${INSTALL_DIR}/${BINARY_NAME}" & spinner
  
  echo -e "\n${BOLD}${GREEN}♻️  Uninstallation Complete!${NC}${NORMAL}"
}

# Main function
main() {
  print_banner
  check_dependencies
  
  case "$1" in
    install)
      install_runtime
      ;;
    uninstall)
      uninstall_runtime
      ;;
    *)
      echo -e "${RED}Usage: $0 [install|uninstall]${NC}"
      exit 1
      ;;
  esac
}

# Handle Ctrl+C
trap "echo -e '\n${RED}✗ Installation aborted!${NC}'; rm -rf $TMP_DIR; exit 1" SIGINT

# Run main function with arguments
main "$@"
