#!/usr/bin/env bash
# install.sh – zero-config, auto-detecting, colourful installer
set -euo pipefail

# ------------------------------------------------------------------ #
#  CONFIG
# ------------------------------------------------------------------ #
REPO_URL="git@github.com:Zer0C0d3/Runtime.git"
INSTALL_DIR="/usr/local/bin"
TMP_DIR="/tmp/runtime-install-$$"
BINARY_NAME="runtime"          # default fallback
QUIET_BUILD=true               # set to false to see full build log

# ------------------------------------------------------------------ #
#  COLOURS & ICONS  (Nerd-Font)
# ------------------------------------------------------------------ #
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[0;33m'
readonly CYAN='\033[0;36m'
readonly MAGENTA='\033[0;35m'
readonly BOLD=$(tput bold)
readonly NC='\033[0m'

readonly ICON_ARROW="➜"
readonly ICON_CHECK="󰗠"
readonly ICON_BOX="󰉏"
readonly ICON_HAMMER="󰛯"
readonly ICON_BIN="󰆴"
readonly ICON_PARTY="󰧂"
readonly ICON_ERROR="󰅖"
readonly ICON_SEARCH="󰍉"

# ------------------------------------------------------------------ #
#  BANNER
# ------------------------------------------------------------------ #
banner() {
cat <<EOF
${MAGENTA}
  _____             _   _
 |  __ \           | | (_)
 | |__) |   _ _ __ | |_ _ _ __ ___   ___
 |  _  / | | | '_ \| __| | '_ \| \| / _ \\
 | | \ \ |_| | | | | |_| | | | | |_| |  __/
 |_|  \_\__,_|_| |_|\__|_|_| |_|\__|\___|
${NC}
EOF
}

# ------------------------------------------------------------------ #
#  SPINNER
# ------------------------------------------------------------------ #
spinner() {
    local pid=$1 msg=$2
    local -a frames=("󰝤" "󰟤" "󰟥" "󰟦" "󰟧" "󰟨")
    local i=0
    while kill -0 "$pid" 2>/dev/null; do
        printf '\r%b %b' "${CYAN}${frames[$i]}${NC}" "$msg"
        i=$(((i + 1) % ${#frames[@]}))
        sleep 0.12
    done
    printf '\r%b %b\n' "${GREEN}${ICON_CHECK}${NC}" "$msg"
}

# ------------------------------------------------------------------ #
#  AUTO-DETECT & BUILD
# ------------------------------------------------------------------ #
detect_and_build() {
    local root=$1
    local build_cmd=""
    local out_dir=""
    local final_binary=""

    # 1. Rust / Cargo -------------------------------------------------
    if [[ -f "$root/Cargo.toml" ]]; then
        build_cmd="cargo build --release"
        out_dir="$root/target/release"
        final_binary="${out_dir}/${BINARY_NAME:-$(toml_get_bin_name "$root/Cargo.toml")}"

    # 2. Makefile -----------------------------------------------------
    elif [[ -f "$root/Makefile" ]]; then
        build_cmd="make"
        out_dir="$root"
        final_binary="$root/${BINARY_NAME}"

    # 3. CMake --------------------------------------------------------
    elif [[ -f "$root/CMakeLists.txt" ]]; then
        build_cmd="cmake -B build -S . && cmake --build build --parallel"
        out_dir="$root/build"
        final_binary="$root/build/${BINARY_NAME}"

    # 4. Node / npm ---------------------------------------------------
    elif [[ -f "$root/package.json" ]]; then
        BINARY_NAME=$(jq -r '.bin | if type=="string" then . else keys[0] end' "$root/package.json" 2>/dev/null || echo "$BINARY_NAME")
        build_cmd="npm ci && npm run build"
        out_dir="$root/dist"
        final_binary="$root/dist/${BINARY_NAME:-index.js}"   # fallback to index.js

    # 5. Go -----------------------------------------------------------
    elif [[ -f "$root/go.mod" ]]; then
        build_cmd="go build -o ${BINARY_NAME}"
        out_dir="$root"
        final_binary="$root/${BINARY_NAME}"

    else
        echo -e "${RED}${ICON_ERROR} Unknown project layout – please add a build rule${NC}"
        exit 1
    fi

    # Build quietly or loudly
    if [[ "${QUIET_BUILD}" == "true" ]]; then
        $build_cmd >/dev/null 2>&1 &
    else
        $build_cmd &
    fi
    spinner $! "Built project"

    # Resolve actual binary (handle host-triple subdirs for Rust)
    if [[ -d "$out_dir" ]]; then
        find "$out_dir" -type f -name "${BINARY_NAME}*" -executable | head -n1
    else
        echo "$final_binary"
    fi
}

# ------------------------------------------------------------------ #
#  TOML helper (Rust binary name)
# ------------------------------------------------------------------ #
toml_get_bin_name() {
    local cargo_toml=$1
    awk -F'=' '/^\[package\]/ {p=1} p && /name[[:space:]]*=/ {gsub(/[" ]/, "", $2); print $2; exit}' "$cargo_toml"
}

# ------------------------------------------------------------------ #
#  INSTALL
# ------------------------------------------------------------------ #
install_runtime() {
    banner

    echo -e "${YELLOW}${ICON_ARROW} Checking privileges…${NC}"
    sudo -v || { echo -e "${RED}${ICON_ERROR} sudo required${NC}"; exit 1; }

    echo -e "${YELLOW}${ICON_ARROW} Checking tools…${NC}"
    for cmd in git cargo make cmake npm node go; do
        command -v "$cmd" >/dev/null && echo -e "${GREEN}${ICON_CHECK} $cmd${NC}"
    done
    echo

    echo -e "${CYAN}${ICON_BOX} Cloning repository…${NC}"
    git clone --depth=1 "$REPO_URL" "$TMP_DIR" >/dev/null 2>&1 &
    spinner $! "Cloned repository"

    echo -e "${CYAN}${ICON_HAMMER} Detecting & building…${NC}"
    BINARY_PATH=$(detect_and_build "$TMP_DIR")

    [[ -f "$BINARY_PATH" ]] || {
        echo -e "${RED}${ICON_ERROR} No executable generated${NC}"
        exit 1
    }

    echo -e "${CYAN}${ICON_BIN} Installing to ${INSTALL_DIR}…${NC}"
    sudo install -m755 "$BINARY_PATH" "$INSTALL_DIR/${BINARY_NAME}" &
    spinner $! "Installed binary"

    rm -rf "$TMP_DIR"
    echo -e "\n${GREEN}${ICON_PARTY} All done! Run ${BOLD}${BINARY_NAME}${NC} --help to start.\n"
}

# ------------------------------------------------------------------ #
#  UNINSTALL
# ------------------------------------------------------------------ #
uninstall_runtime() {
    banner
    [[ ! -f "$INSTALL_DIR/${BINARY_NAME}" ]] && {
        echo -e "${YELLOW}${ICON_ARROW} Nothing to remove – ${BINARY_NAME} is not installed${NC}"
        exit 0
    }

    sudo -v || { echo -e "${RED}${ICON_ERROR} sudo required${NC}"; exit 1; }

    echo -e "${CYAN}${ICON_BIN} Removing ${BINARY_NAME}…${NC}"
    sudo rm -f "$INSTALL_DIR/${BINARY_NAME}" &
    spinner $! "Uninstalled"

    echo -e "\n${GREEN}${ICON_CHECK} Uninstalled – bye for now!\n"
}

# ------------------------------------------------------------------ #
#  MAIN
# ------------------------------------------------------------------ #
case "${1:-}" in
    install)   install_runtime ;;
    uninstall) uninstall_runtime ;;
    *)
        echo -e "${RED}${ICON_ERROR} Usage: sudo $0 [install|uninstall]${NC}"
        exit 1
        ;;
esac