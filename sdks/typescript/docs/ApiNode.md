
# ApiNode


## Properties

Name | Type
------------ | -------------
`name` | string
`url` | string
`vcpuTotal` | number
`vcpuAvailable` | number
`memoryTotal` | number
`memoryAvailable` | number
`diskTotal` | number
`diskAvailable` | number
`specialization` | string
`status` | [ApiNodeStatus](ApiNodeStatus.md)
`createdAt` | Date
`updatedAt` | Date

## Example

```typescript
import type { ApiNode } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "name": null,
  "url": null,
  "vcpuTotal": null,
  "vcpuAvailable": null,
  "memoryTotal": null,
  "memoryAvailable": null,
  "diskTotal": null,
  "diskAvailable": null,
  "specialization": null,
  "status": null,
  "createdAt": null,
  "updatedAt": null,
} satisfies ApiNode

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiNode
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


