#!/bin/bash
set -e

# Merge all repos inside current working directory into one repo (this folder itself)

# Ensure we are not inside an existing repo
if [ -d ".git" ]; then
  echo "❌ This folder is already a git repo. Please run in a non-git parent folder."
  exit 1
fi

echo "🚀 Initializing new unified repo in: $(pwd)"
git init

# Loop through all subfolders
for dir in */; do
  if [ -d "$dir/.git" ]; then
    echo "📂 Found repo: $dir"
    cd "$dir"

    # Ensure filter-repo is installed
    if ! command -v git-filter-repo &> /dev/null; then
      echo "❌ Error: git-filter-repo not installed."
      echo "👉 Install it: https://github.com/newren/git-filter-repo"
      exit 1
    fi

    # Rewrite history so files live under subdir
    git filter-repo --to-subdirectory-filter "${dir%/}" --force

    cd ..
    # Merge history into unified repo
    git remote add temp "$dir"
    git fetch temp
    git merge temp/HEAD --allow-unrelated-histories -m "Merge $dir"
    git remote remove temp

    # Delete old .git (make it plain folder now)
    rm -rf "$dir/.git"
  fi
done

echo "✅ All repos merged into $(pwd)"
echo "👉 Next steps:"
echo "   git remote add origin <your-github-repo-url>"
echo "   git branch -M main"
echo "   git push -u origin main"

