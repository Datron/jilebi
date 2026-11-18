
# ApiNodeMaintenanceEntry


## Properties

Name | Type
------------ | -------------
`username` | string
`machineName` | string
`ipAddress` | string
`prevState` | string
`state` | string
`restored` | boolean
`destinationUrl` | string
`error` | string

## Example

```typescript
import type { ApiNodeMaintenanceEntry } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "username": null,
  "machineName": null,
  "ipAddress": null,
  "prevState": null,
  "state": null,
  "restored": null,
  "destinationUrl": null,
  "error": null,
} satisfies ApiNodeMaintenanceEntry

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiNodeMaintenanceEntry
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


