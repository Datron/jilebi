
# ApiVmSleepResponse


## Properties

Name | Type
------------ | -------------
`success` | boolean
`destinationUrl` | string
`alreadySleeping` | boolean
`pauseMs` | number
`snapshotMs` | number
`totalMs` | number
`deduped` | boolean
`stateChanged` | boolean

## Example

```typescript
import type { ApiVmSleepResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "success": null,
  "destinationUrl": null,
  "alreadySleeping": null,
  "pauseMs": null,
  "snapshotMs": null,
  "totalMs": null,
  "deduped": null,
  "stateChanged": null,
} satisfies ApiVmSleepResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmSleepResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


