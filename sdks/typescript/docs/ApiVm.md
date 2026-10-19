
# ApiVm


## Properties

Name | Type
------------ | -------------
`id` | string
`userId` | string
`machineName` | string
`networkName` | string
`nodeName` | string
`hypervisorSocket` | string
`ipAddress` | string
`cpuCount` | number
`memorySize` | number
`diskSizeGb` | number
`state` | [ApiVmState](ApiVmState.md)
`createdAt` | Date
`updatedAt` | Date

## Example

```typescript
import type { ApiVm } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "userId": null,
  "machineName": null,
  "networkName": null,
  "nodeName": null,
  "hypervisorSocket": null,
  "ipAddress": null,
  "cpuCount": null,
  "memorySize": null,
  "diskSizeGb": null,
  "state": null,
  "createdAt": null,
  "updatedAt": null,
} satisfies ApiVm

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVm
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


