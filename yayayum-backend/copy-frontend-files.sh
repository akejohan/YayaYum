#!/usr/bin/env bash

# Check that we are in the backend directory
if [ ! -f "Cargo.toml" ]; then
  echo "Error: This script must be run from the backend directory."
  exit 1
fi

# Clear existing static files
rm -rf assets || exit

# Build frontend files
cd ../yayayum-frontend || exit 1
npm run build || exit 1

# Copy built files to backend static directory
cd ../yayayum-backend || exit 1
cp -r ../yayayum-frontend/dist assets || exit 1

echo "Frontend files copied successfully."