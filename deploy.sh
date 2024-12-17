#!/bin/bash

# Move to the directory where the script is located
cd "$(dirname "$0")"

# Pre-defined variables
acr_name="recommenderregistry"
tag="latest"

# Function to check the status of the last command
check_error() {
    if [ $? -ne 0 ]; then
        echo "An error occurred. Exiting..."
        read -n 1 -s -r -p "Press any key to close..."
        exit 1
    fi
}

# Function to build, tag, and push Docker images
deploy_image() {
    local image_dir=$1
    local image_name=$2

    echo "Building Docker image for $image_name..."
    cd $image_dir
    check_error

    # Load .env variables, ignoring lines with missing values or comments
    # Load .env variables, ignoring comments
	if [ -f .env ]; then
		set -a
		source <(grep -v '^#' .env)
		set +a
	fi


    docker build --no-cache \
        --build-arg APIBASE="${APIBASE:-}" \
        --build-arg COSMOS_DB_URL="${COSMOS_DB_URL:-}" \
        --build-arg COSMOS_DB_KEY="${COSMOS_DB_KEY:-}" \
        --build-arg DATABASE_NAME="${DATABASE_NAME:-}" \
        --build-arg CONTAINER_NAME="${CONTAINER_NAME:-}" \
        -t $image_name:$tag .
    check_error
	
	# Build the Docker image without using the cache
	echo "Building Docker image without cache..."
	docker build --no-cache -t $image_name:$tag .
	check_error

	# Log in to Azure Container Registry
	echo "Logging into ACR: $acr_name..."
	az acr login --name $acr_name
	check_error

    echo "Tagging image for ACR..."
    docker tag $image_name:$tag $acr_name.azurecr.io/$image_name:$tag
    check_error

    echo "Pushing Docker image to ACR..."
    docker push $acr_name.azurecr.io/$image_name:$tag
    check_error

    echo "Docker image for $image_name pushed successfully!"
    cd ..
}

# Build and deploy movienight-backend
#deploy_image "backend" "movienight-backend"

# Build and deploy movienight-frontend
deploy_image "frontend" "movienight-frontend"

# Build and deploy movienight-recommender
#deploy_image "recommender_system" "movienight-recommender"

# Clean up dangling images locally
echo "Cleaning up dangling images..."
docker image prune -f
check_error

# Optionally, clean up old images in ACR
echo "Cleaning up older images in ACR..."
for repo in "movienight-recommender" "movienight-frontend" "movienight-backend"; do
    az acr repository show-tags --name $acr_name --repository $repo --orderby time_desc --output tsv | tail -n +3 | while read image; do
        az acr repository delete --name $acr_name --image $repo:$image --yes
        check_error
    done
done

echo "All images deployed and cleaned up successfully!"

# Wait for a key press before exiting
read -n 1 -s -r -p "Press any key to close..."
