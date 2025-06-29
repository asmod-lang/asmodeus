#!/usr/bin/env bash
# development wrapper - asmod through cargo for development

BOLD='\033[1m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BOLD}${BLUE}🔧 Development mode - running through cargo...${NC}"
cargo run -- "$@"
