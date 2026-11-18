
# ApiVmMetricsResponse


## Properties

Name | Type
------------ | -------------
`machineName` | string
`nodeName` | string
`range` | string
`start` | number
`end` | number
`stepSeconds` | number
`available` | boolean
`error` | string
`configured` | [ApiVmMetricsResponseConfigured](ApiVmMetricsResponseConfigured.md)
`cpu` | [ApiVmMetricsResponseCpu](ApiVmMetricsResponseCpu.md)
`memory` | [ApiVmMetricsResponseMemory](ApiVmMetricsResponseMemory.md)
`network` | [ApiVmMetricsResponseNetwork](ApiVmMetricsResponseNetwork.md)
`disk` | [ApiVmMetricsResponseDisk](ApiVmMetricsResponseDisk.md)

## Example

```typescript
import type { ApiVmMetricsResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "machineName": null,
  "nodeName": null,
  "range": null,
  "start": null,
  "end": null,
  "stepSeconds": null,
  "available": null,
  "error": null,
  "configured": null,
  "cpu": null,
  "memory": null,
  "network": null,
  "disk": null,
} satisfies ApiVmMetricsResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmMetricsResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


