param serverName string
param location string
param adminUsername string
param adminPassword string

resource postgresServer 'Microsoft.DBforPostgreSQL/servers@2021-06-01' = {
  name: serverName
  location: location
  properties: {
    administratorLogin: adminUsername
    administratorLoginPassword: adminPassword
    version: '11'
    sslEnforcement: 'Enabled'
    storageProfile: {
      storageMB: 5120
      backupRetentionDays: 7
      geoRedundantBackup: 'Disabled'
    }
  }
}
