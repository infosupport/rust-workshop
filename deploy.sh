#!/bin/bash

# Obtain adminUsername and adminPassword from the command line
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --adminUsername) adminUsername="$2"; shift ;;
        --adminPassword) adminPassword="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# Create a new resource group if it doesn't already exist
az group create --name rg-rustworkshop-neu --location NorthEurope

# Deploy the main.bicep file using Azure CLI
az deployment group create \
  --resource-group rg-rustworkshop-neu \
  --template-file iac/main.bicep \
  --parameters adminUsername=$adminUsername adminPassword=$adminPassword @iac/main.parameters.json
