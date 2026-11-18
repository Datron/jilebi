
# ApiNodeMaintenanceStatus


## Properties

Name | Type
------------ | -------------
`node` | string
`phase` | string
`running` | boolean
`startedAt` | Date
`routesPending` | boolean
`entries` | [Array&lt;ApiNodeMaintenanceEntry&gt;](ApiNodeMaintenanceEntry.md)

## Example

```typescript
import type { ApiNodeMaintenanceStatus } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "node": null,
  "phase": null,
  "running": null,
  "startedAt": null,
  "routesPending": null,
  "entries": null,
} satisfies ApiNodeMaintenanceStatus

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiNodeMaintenanceStatus
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


