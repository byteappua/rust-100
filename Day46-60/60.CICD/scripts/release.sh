#!/bin/bash
# Release script for semantic versioning

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: ./release.sh <version>"
    echo "Example: ./release.sh 0.2.0"
    exit 1
fi

VERSION=$1

# Validate semantic version format
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version must be in format: MAJOR.MINOR.PATCH (e.g., 0.2.0)"
    exit 1
fi

echo -e "${GREEN}Starting release process for v${VERSION}${NC}"

# Check if working directory is clean
if [[ -n $(git status -s) ]]; then
    echo -e "${YELLOW}Warning: Working directory is not clean${NC}"
    git status -s
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Update version in Cargo.toml
echo -e "${GREEN}Updating Cargo.toml version...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
rm Cargo.toml.bak

# Generate changelog (requires git-cliff)
if command -v git-cliff &> /dev/null; then
    echo -e "${GREEN}Generating CHANGELOG...${NC}"
    git-cliff -o CHANGELOG.md
else
    echo -e "${YELLOW}Warning: git-cliff not found, skipping changelog generation${NC}"
fi

# Run tests
echo -e "${GREEN}Running tests...${NC}"
cargo test

# Commit changes
echo -e "${GREEN}Committing changes...${NC}"
git add Cargo.toml CHANGELOG.md
git commit -m "chore: release v${VERSION}"

# Create tag
echo -e "${GREEN}Creating tag v${VERSION}...${NC}"
git tag -a "v${VERSION}" -m "Release v${VERSION}"

echo -e "${GREEN}Release prepared successfully!${NC}"
echo -e "${YELLOW}To complete the release, run:${NC}"
echo "  git push origin main --tags"
