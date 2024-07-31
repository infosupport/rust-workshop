param environmentName string
param location string
param serverName string
param adminUsername string
param adminPassword string
param registryName string
param workspaceName string

module logAnalytics 'log-analytics.bicep' = {
  name: 'logAnalytics'
  params: {
    workspaceName: workspaceName
    location: location
  }
}

module containerApp 'container-app.bicep' = {
  name: 'containerApp'
  params: {
    environmentName: environmentName
    location: location
    customerId: logAnalytics.outputs.customerId
    sharedKey: logAnalytics.outputs.sharedKey
  }
}

module postgres 'postgres.bicep' = {
  name: 'postgres'
  params: {
    serverName: serverName
    location: location
    adminUsername: adminUsername
    adminPassword: adminPassword
  }
}

module containerRegistry 'container-registry.bicep' = {
  name: 'containerRegistry'
  params: {
    registryName: registryName
    location: location
  }
}

output containerAppEnvironmentId string = containerApp.outputs.containerAppEnvironmentId
output postgresServerId string = postgres.outputs.postgresServerId
output containerRegistryId string = containerRegistry.outputs.containerRegistryId
output logAnalyticsWorkspaceId string = logAnalytics.outputs.logAnalyticsWorkspaceId
