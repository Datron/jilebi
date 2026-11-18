
# ApiNodeMaintenanceRunResponse


## Properties

Name | Type
------------ | -------------
`node` | string
`phase` | string
`running` | boolean
`startedAt` | Date
`routesPending` | boolean
`entries` | [Array&lt;ApiNodeMaintenanceEntry&gt;](ApiNodeMaintenanceEntry.md)
`message` | string

## Example

```typescript
import type { ApiNodeMaintenanceRunResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "node": null,
  "phase": null,
  "running": null,
  "startedAt": null,
  "routesPending": null,
  "entries": null,
  "message": null,
} satisfies ApiNodeMaintenanceRunResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiNodeMaintenanceRunResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


