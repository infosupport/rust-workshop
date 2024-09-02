param environmentName string
param location string
param databaseServerName string
param adminUsername string
@secure()
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
    logAnalyticsWorkspaceName: workspaceName
  }
}

module postgres 'postgres.bicep' = {
  name: 'postgres'
  params: {
    serverName: databaseServerName
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
