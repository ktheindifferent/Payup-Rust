#!/bin/bash

# Credential Scanner Script
# This script scans for potential leaked credentials in the codebase
# Exit with status 1 if potential secrets are found

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔍 Scanning for potential leaked credentials..."

# Counter for found issues
FOUND_ISSUES=0

# Files to scan (exclude binary files and dependencies)
FILES_TO_SCAN=$(git ls-files | grep -v -E '(target/|\.git/|\.env\.example|SECURITY\.md|scan-secrets\.sh|pre-commit|Cargo\.lock)')

# Patterns to search for
declare -a PATTERNS=(
    # Stripe keys
    'sk_test_[a-zA-Z0-9]{24,}'
    'sk_live_[a-zA-Z0-9]{24,}'
    'pk_test_[a-zA-Z0-9]{24,}'
    'pk_live_[a-zA-Z0-9]{24,}'
    'whsec_[a-zA-Z0-9]{24,}'
    
    # PayPal
    'client_id.*=.*["\047][A-Za-z0-9]{20,}["\047]'
    'client_secret.*=.*["\047][A-Za-z0-9]{20,}["\047]'
    
    # Square
    'sq0atp-[a-zA-Z0-9\-_]{22,}'
    'sq0csp-[a-zA-Z0-9\-_]{22,}'
    'EAAA[a-zA-Z0-9]{20,}'
    
    # Generic API keys (with actual values, not placeholders)
    'api_key.*=.*["\047](?!PLACEHOLDER|YOUR_|placeholder|your_|example|test_key)[a-zA-Z0-9]{20,}["\047]'
    'apikey.*=.*["\047](?!PLACEHOLDER|YOUR_|placeholder|your_|example|test_key)[a-zA-Z0-9]{20,}["\047]'
    'access_token.*=.*["\047](?!PLACEHOLDER|YOUR_|placeholder|your_|example|test_token)[a-zA-Z0-9]{20,}["\047]'
    'secret.*=.*["\047](?!PLACEHOLDER|YOUR_|placeholder|your_|example|test_secret)[a-zA-Z0-9]{20,}["\047]'
    
    # Private keys
    '-----BEGIN RSA PRIVATE KEY-----'
    '-----BEGIN EC PRIVATE KEY-----'
    '-----BEGIN OPENSSH PRIVATE KEY-----'
    '-----BEGIN DSA PRIVATE KEY-----'
    '-----BEGIN PRIVATE KEY-----'
    
    # AWS
    'AKIA[0-9A-Z]{16}'
    'aws_secret_access_key.*=.*[a-zA-Z0-9/+=]{40}'
    
    # GitHub
    'ghp_[a-zA-Z0-9]{36}'
    'gho_[a-zA-Z0-9]{36}'
    'ghu_[a-zA-Z0-9]{36}'
    'ghs_[a-zA-Z0-9]{36}'
    'ghr_[a-zA-Z0-9]{36}'
)

# Function to check a pattern
check_pattern() {
    local pattern=$1
    local results
    
    # Use grep -P for Perl regex if available, otherwise use extended regex
    if command -v ggrep >/dev/null 2>&1; then
        # macOS with GNU grep installed
        results=$(echo "$FILES_TO_SCAN" | xargs ggrep -l -P "$pattern" 2>/dev/null || true)
    elif grep -P "" /dev/null 2>/dev/null; then
        # Linux with Perl regex support
        results=$(echo "$FILES_TO_SCAN" | xargs grep -l -P "$pattern" 2>/dev/null || true)
    else
        # Fallback to extended regex (less accurate but more compatible)
        results=$(echo "$FILES_TO_SCAN" | xargs grep -l -E "$pattern" 2>/dev/null || true)
    fi
    
    if [ -n "$results" ]; then
        echo -e "${RED}⚠️  Found potential secret matching pattern: $pattern${NC}"
        echo "$results" | while read -r file; do
            echo "   📄 $file"
            FOUND_ISSUES=$((FOUND_ISSUES + 1))
        done
        return 1
    fi
    return 0
}

# Check for common hardcoded test credentials (less strict patterns)
echo "Checking for hardcoded credentials..."

# Simple checks that work on all systems
for file in $FILES_TO_SCAN; do
    # Skip binary files (check if file command exists)
    if command -v file >/dev/null 2>&1; then
        if file "$file" | grep -q "binary"; then
            continue
        fi
    fi
    
    # Check for obvious Stripe keys
    if grep -q "sk_test_" "$file" 2>/dev/null && ! grep -q "PLACEHOLDER\|YOUR_\|example" "$file" 2>/dev/null; then
        # Check if it's not just a placeholder
        if grep "sk_test_" "$file" | grep -v "sk_test_PLACEHOLDER\|sk_test_YOUR\|sk_test_example\|sk_test_\"\|sk_test_'" | grep -q "sk_test_[a-zA-Z0-9]"; then
            echo -e "${RED}⚠️  Potential Stripe test key in: $file${NC}"
            FOUND_ISSUES=$((FOUND_ISSUES + 1))
        fi
    fi
    
    if grep -q "sk_live_" "$file" 2>/dev/null; then
        echo -e "${RED}🚨 CRITICAL: Potential Stripe LIVE key in: $file${NC}"
        FOUND_ISSUES=$((FOUND_ISSUES + 1))
    fi
    
    if grep -q "whsec_" "$file" 2>/dev/null && ! grep -q "PLACEHOLDER\|YOUR_\|example" "$file" 2>/dev/null; then
        if grep "whsec_" "$file" | grep -v "whsec_PLACEHOLDER\|whsec_YOUR\|whsec_test_secret" | grep -q "whsec_[a-zA-Z0-9]"; then
            echo -e "${RED}⚠️  Potential webhook secret in: $file${NC}"
            FOUND_ISSUES=$((FOUND_ISSUES + 1))
        fi
    fi
    
    # Check for Square tokens
    if grep -q "sq0atp-\|sq0csp-\|EAAA" "$file" 2>/dev/null; then
        echo -e "${RED}⚠️  Potential Square token in: $file${NC}"
        FOUND_ISSUES=$((FOUND_ISSUES + 1))
    fi
    
    # Check for private keys
    if grep -q "BEGIN.*PRIVATE KEY" "$file" 2>/dev/null; then
        echo -e "${RED}🚨 CRITICAL: Private key found in: $file${NC}"
        FOUND_ISSUES=$((FOUND_ISSUES + 1))
    fi
done

# Check for .env file that shouldn't be committed
if [ -f ".env" ] && git ls-files --error-unmatch .env 2>/dev/null; then
    echo -e "${RED}🚨 CRITICAL: .env file is tracked by git! It should be in .gitignore${NC}"
    FOUND_ISSUES=$((FOUND_ISSUES + 1))
fi

# Summary
echo ""
if [ $FOUND_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✅ No potential secrets found!${NC}"
    exit 0
else
    echo -e "${RED}❌ Found $FOUND_ISSUES potential security issue(s)!${NC}"
    echo ""
    echo "Please review the files above and ensure:"
    echo "1. No real API keys or secrets are hardcoded"
    echo "2. All credentials are loaded from environment variables"
    echo "3. Only use placeholder values in example code"
    echo "4. Add .env to .gitignore if not already done"
    echo ""
    echo "If these are false positives (e.g., example placeholders), you can:"
    echo "- Use clearly fake values like 'sk_test_PLACEHOLDER'"
    echo "- Load from environment variables instead"
    echo ""
    exit 1
fi