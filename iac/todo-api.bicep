param location string
param containerRegistryName string
param databaseServerName string
@secure()
param databasePassword string
param databaseUserName string
param containerAppsEnvironmentName string

resource containerRegistry 'Microsoft.ContainerRegistry/registries@2023-11-01-preview' existing = {
  name: containerRegistryName
}

resource postgresServer 'Microsoft.DBforPostgreSQL/flexibleServers@2023-12-01-preview' existing = {
  name: databaseServerName
}

resource containerAppEnvironment 'Microsoft.App/managedEnvironments@2024-03-01' existing = {
  name: containerAppsEnvironmentName
}

resource todoApiApplication 'Microsoft.App/containerApps@2024-03-01' = {
  location: location
  name: 'todo-api'
  identity: { type: 'SystemAssigned' }
  properties: {
    environmentId: containerAppEnvironment.id
    configuration: {
      activeRevisionsMode: 'Single'
      ingress: {
        allowInsecure: false
        targetPort: 3000
      }
      registries: [
        {
          passwordSecretRef: 'registry-password'
          server: containerRegistry.properties.loginServer
          username: containerRegistry.name
        }
      ]
      secrets: [
        {
          name: 'registry-password'
          value: containerRegistry.listCredentials().passwords[0].value
        }
        {
          name: 'database-username'
          value: databaseUserName
        }
        {
          name: 'database-password'
          value: databasePassword
        }
      ]
    }
    template: {
      containers: [
        {
          name: 'todo-api'
          image: '${containerRegistry.properties.loginServer}/api:1'
          resources: { cpu: 1, memory: '2.0Gi' }
          env: [
            { name: 'APP_DATABASE_HOST', value: postgresServer.properties.fullyQualifiedDomainName }
            { name: 'APP_DATABASE_PORT', value: '5432' }
            { name: 'APP_DATABASE_NAME', value: 'todo_api' }
            { name: 'APP_DATABASE_USER', secretRef: 'database-username' }
            { name: 'APP_DATABASE_PASSWORD', secretRef: 'database-password' }
          ]
        }
      ]
    }
  }
}
