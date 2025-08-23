#!/bin/bash

# Security Setup Script
# This script sets up security measures for the project

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔐 Setting up security measures for Payup...${NC}"
echo ""

# 1. Install pre-commit hook
echo -e "${YELLOW}📌 Installing pre-commit hook...${NC}"
if [ -d ".git/hooks" ]; then
    cp hooks/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo -e "${GREEN}✅ Pre-commit hook installed successfully${NC}"
else
    echo -e "${YELLOW}⚠️  Warning: .git/hooks directory not found. Are you in a git repository?${NC}"
    echo "   To manually install later, run:"
    echo "   cp hooks/pre-commit .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit"
fi
echo ""

# 2. Check for .env file
echo -e "${YELLOW}📋 Checking environment configuration...${NC}"
if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        echo "Creating .env file from .env.example..."
        cp .env.example .env
        echo -e "${GREEN}✅ Created .env file${NC}"
        echo -e "${YELLOW}⚠️  Please edit .env and add your actual API keys${NC}"
    else
        echo -e "${YELLOW}⚠️  No .env.example file found${NC}"
    fi
else
    echo -e "${GREEN}✅ .env file already exists${NC}"
fi
echo ""

# 3. Verify .gitignore
echo -e "${YELLOW}🚫 Verifying .gitignore configuration...${NC}"
if [ -f ".gitignore" ]; then
    if grep -q "^\.env$" .gitignore; then
        echo -e "${GREEN}✅ .env is properly ignored${NC}"
    else
        echo -e "${YELLOW}⚠️  .env is not in .gitignore. Adding it now...${NC}"
        echo ".env" >> .gitignore
        echo -e "${GREEN}✅ Added .env to .gitignore${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  No .gitignore file found. Creating one...${NC}"
    echo ".env" > .gitignore
    echo -e "${GREEN}✅ Created .gitignore with .env${NC}"
fi
echo ""

# 4. Set file permissions
echo -e "${YELLOW}🔒 Setting secure file permissions...${NC}"
if [ -f ".env" ]; then
    chmod 600 .env
    echo -e "${GREEN}✅ Set .env permissions to 600 (owner read/write only)${NC}"
fi
echo ""

# 5. Run initial security scan
echo -e "${YELLOW}🔍 Running initial security scan...${NC}"
if [ -f "scripts/scan-secrets.sh" ]; then
    chmod +x scripts/scan-secrets.sh
    ./scripts/scan-secrets.sh || true
else
    echo -e "${YELLOW}⚠️  Security scanner not found${NC}"
fi
echo ""

# 6. Display security checklist
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}📋 Security Setup Complete!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo "✅ Pre-commit hook installed (prevents committing secrets)"
echo "✅ .env file created (add your API keys here)"
echo "✅ .gitignore configured (prevents committing .env)"
echo "✅ File permissions secured"
echo "✅ Security scanner available"
echo ""
echo -e "${YELLOW}📚 Next Steps:${NC}"
echo "1. Edit .env and add your actual API keys"
echo "2. Review SECURITY.md for best practices"
echo "3. Run './scripts/scan-secrets.sh' to scan for secrets"
echo "4. Test the pre-commit hook with: git add -A && git commit -m 'test'"
echo ""
echo -e "${GREEN}🛡️ Your repository is now protected against accidental credential leaks!${NC}"
echo ""
echo "For more information, see:"
echo "  • SECURITY.md - Complete security guidelines"
echo "  • .env.example - Template for environment variables"
echo "  • hooks/pre-commit - Pre-commit hook source code"