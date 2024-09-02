#!/bin/bash

# Obtain databaseUserName and databasePassword from the command line
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --databaseUserName) databaseUserName="$2"; shift ;;
        --databasePassword) databasePassword="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Check that databaseUserName and databasePassword are set
if [ -z "$databasePassword" ]; then
    echo "databasePassword is required"
    exit 1
fi

if [ -z "$databaseUserName" ]; then
    echo "databaseUsername is required"
    exit 1
fi

# Deploy the todo-api.bicep file using Azure CLI
az deployment group create \
  --resource-group rg-rustworkshop-neu \
  --template-file iac/todo-api.bicep \
  --parameters databaseUserName=$databaseUserName databasePassword=$databasePassword @iac/todo-api.parameters.json
