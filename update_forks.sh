#!/bin/bash
set -e

# Function to update and create missing branches if needed
update_repo() {
    local repo_dir=$1
    local upstream_url=$2
    local target_branch=$3

    echo "----------------------------------------"
    echo "Updating repository at: $repo_dir"
    
    if [ ! -d "$repo_dir" ]; then
        echo "Error: Directory $repo_dir does not exist."
        return 1
    fi

    cd "$repo_dir" || { echo "Failed to cd into $repo_dir"; exit 1; }

    # Check if upstream remote already exists
    if ! git remote | grep -q "^upstream$"; then
        echo "Adding upstream remote: $upstream_url"
        git remote add upstream "$upstream_url"
    else
        echo "Upstream remote already exists."
    fi

    echo "Fetching specific branches from upstream and origin..."
    git fetch upstream "$target_branch"
    git fetch origin "$target_branch"

    # Get current branch name
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    echo "Current branch: $current_branch"

    # If we are not on the target branch, try to switch or create it tracking upstream
    if [ "$current_branch" != "$target_branch" ]; then
        if git show-ref --verify --quiet "refs/heads/$target_branch"; then
            echo "Switching to existing local branch '$target_branch'..."
            git checkout "$target_branch"
        else
            echo "Creating new branch '$target_branch' tracking origin/$target_branch..."
            git checkout -b "$target_branch" "origin/$target_branch"
        fi
    fi

    # Clean up any interrupted merges
    if [ -f ".git/MERGE_HEAD" ]; then
        echo "Previous merge was interrupted. Aborting..."
        git merge --abort
    fi

    # Sync with origin first to avoid non-fast-forward errors
    echo "Merging origin/$target_branch into $target_branch..."
    git merge --no-edit "origin/$target_branch"

    # Merge upstream changes
    echo "Merging upstream/$target_branch into $target_branch..."
    git merge --no-edit "upstream/$target_branch"

    # Push to fork
    echo "Pushing updated $target_branch to origin..."
    git push origin "$target_branch"

    # Return to previous directory
    cd - > /dev/null
    echo "Repository update complete."
}


# 1. Update tao (branch: dev)  - Tao uses 'dev' as the main development branch
# relative path from within chenchen/ to ../tao
update_repo "../tao" "https://github.com/tauri-apps/tao.git" "dev"

# 2. Update tauri (branch: dev) - Tauri dev branch is usually the main default
# relative path from within chenchen/ to ../tauri
update_repo "../tauri" "https://github.com/tauri-apps/tauri.git" "dev"

# 3. Update chenchen dependencies
echo "----------------------------------------"
echo "Updating chenchen dependencies via cargo..."
if [ -d "src-tauri" ]; then
    cd src-tauri
    # Updates tauri and tao specifically
    # Use git CLI for fetch to avoid libgit2 authentication issues
    CARGO_NET_GIT_FETCH_WITH_CLI=true cargo update -p tauri -p tao
    cd ..
    echo "All updates complete!"
else
    echo "Warning: src-tauri directory not found! Skipping cargo update."
fi
