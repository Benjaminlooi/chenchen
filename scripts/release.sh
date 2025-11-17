#!/bin/bash

# Automated release script for ChenChen
# Usage: ./scripts/release.sh <patch|minor|major>

set -e  # Exit on error

BUMP_TYPE=$1

if [ -z "$BUMP_TYPE" ]; then
  echo "Usage: ./scripts/release.sh <patch|minor|major>"
  exit 1
fi

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting release process...${NC}"

# 1. Check git status
if [ -n "$(git status --porcelain)" ]; then
  echo -e "${YELLOW}Warning: You have uncommitted changes${NC}"
  git status --short
  read -p "Continue anyway? (y/N) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi

# 2. Run tests
echo -e "${YELLOW}Running tests...${NC}"
npm test
cd src-tauri && cargo test && cd ..
echo -e "${GREEN}✓ All tests passed${NC}"

# 3. Bump version
echo -e "${YELLOW}Bumping version...${NC}"
npm run version:$BUMP_TYPE

# Extract new version from package.json
NEW_VERSION=$(node -p "require('./package.json').version")
echo -e "${GREEN}✓ Version bumped to ${NEW_VERSION}${NC}"

# 4. Update changelog
echo -e "${YELLOW}Don't forget to update CHANGELOG.md!${NC}"
read -p "Press enter to continue after updating changelog..."

# 5. Commit changes
echo -e "${YELLOW}Committing version bump...${NC}"
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json CHANGELOG.md
git commit -m "chore: release v${NEW_VERSION}"
echo -e "${GREEN}✓ Changes committed${NC}"

# 6. Create tag
echo -e "${YELLOW}Creating git tag...${NC}"
git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION}"
echo -e "${GREEN}✓ Tag created${NC}"

# 7. Push
echo -e "${YELLOW}Pushing to remote...${NC}"
read -p "Push to remote and trigger release? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  git push && git push --tags
  echo -e "${GREEN}✓ Pushed to remote${NC}"
  echo -e "${GREEN}✓ GitHub Actions will now build and create the release${NC}"
  echo -e "${GREEN}Check: https://github.com/$(git remote get-url origin | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions${NC}"
else
  echo -e "${YELLOW}Skipped push. Run manually with: git push && git push --tags${NC}"
fi

echo -e "${GREEN}Release process complete!${NC}"
