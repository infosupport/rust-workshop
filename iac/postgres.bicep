param serverName string
param location string
param adminUsername string
@secure()
param adminPassword string

resource postgresServer 'Microsoft.DBforPostgreSQL/flexibleServers@2023-12-01-preview' = {
  name: serverName
  location: location
  sku: {
    name: 'Standard_B1ms'
    tier: 'Burstable'
  }
  properties: {
    administratorLogin: adminUsername
    administratorLoginPassword: adminPassword
    version: '16'
    storage: {
      autoGrow: 'Disabled'
      storageSizeGB: 32
      tier: 'P4'
    }
  }

  resource todoApiDatabase 'databases' = {
    name: 'todo_api'
    properties: {
      charset: 'UTF8'
      collation: 'en_US.UTF8'
    }
  }
}
