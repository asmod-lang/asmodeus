#!/usr/bin/env bash

set -e

WHITE='\033[38;5;255m'
LIGHT_GRAY='\033[38;5;252m'
GRAY='\033[38;5;248m'
GRAY_DARK='\033[38;5;244m'
BORDOISE='\033[38;5;52m'
RED_DARK='\033[38;5;88m'
MAROON='\033[38;5;124m'
RED='\033[38;5;160m'

ORANGE='\033[38;5;202m'
YELLOW='\033[38;5;220m'
GREEN='\033[38;5;82m'
PINK='\033[38;5;210m'
PURPLE='\033[38;5;177m'
BOLD='\033[1m'
NC='\033[0m'

echo -e ""
echo -e "${BOLD}${WHITE} ______                                  __     ${NC}"
echo -e "${BOLD}${LIGHT_GRAY}/\  _  \                                /\ \    ${NC}"
echo -e "${BOLD}${GRAY}\ \ \_\ \    ____    ___ ___     ___    \_\ \   ${NC}"
echo -e "${BOLD}${GRAY_DARK} \ \  __ \  /',__\ /' __\` __\`\  / __\`\  /'_\` \  ${NC}"
echo -e "${BOLD}${BORDOISE}  \ \ \/\ \/\__, \`\/\ \/\ \/\ \/\ \_\ \/\ \_\ \ ${NC}"
echo -e "${BOLD}${RED_DARK}   \ \_\ \_\/\____/\ \_\ \_\ \_\ \____/\ \___,_\ ${NC}"
echo -e "${BOLD}${MAROON}    \/_/\/_/\/___/  \/_/\/_/\/_/\/___/  \/__,_ /${NC}"
echo -e ""

echo -e "${BOLD}${ORANGE}🚀 Installing Asmodeus Compiler (asmod)...${NC}"

echo -e "${BOLD}${PURPLE}📦 Building optimized binary...${NC}"
cargo build --release

BINARY_PATH="target/release/asmodeus"
INSTALL_DIR="$HOME/.local/bin"

mkdir -p "$INSTALL_DIR"

echo -e "${BOLD}📥 Installing to $INSTALL_DIR/asmod...${NC}"
cp "$BINARY_PATH" "$INSTALL_DIR/asmod"

# executable
chmod +x "$INSTALL_DIR/asmod"

echo -e ""
echo -e "${BOLD}${GREEN}✅ asmod: Asmodeus Compiler installed successfully!${NC}"
echo -e ""
echo -e "${BOLD}${MAROON} Usage:${NC}"
echo -e "  ${BOLD}asmod run program.asmod${NC}           ${GRAY}# Run assembly program${NC}"
echo -e "  ${BOLD}asmod debug program.asmod${NC}         ${GRAY}# Run with Bugseer (Asmodeus debugger)${NC}"
echo -e "  ${BOLD}asmod run --debug program.asmod${NC}   ${GRAY}# Run with debug output${NC}"
echo -e "  ${BOLD}asmod --help${NC}                      ${GRAY}# Show all options${NC}"
echo -e ""
echo -e "${BOLD}${PURPLE}📝 Make sure $INSTALL_DIR is in your PATH${NC}"
echo -e "   Add this to your ~/.bashrc or ~/.zshrc:"
echo -e "   ${BOLD}${PINK}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
echo -e ""
