param environmentName string
param location string
param serverName string
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
