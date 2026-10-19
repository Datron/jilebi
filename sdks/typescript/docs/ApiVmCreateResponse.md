
# ApiVmCreateResponse


## Properties

Name | Type
------------ | -------------
`networkName` | string
`hypervisorSocket` | string
`id` | string
`nodeName` | string

## Example

```typescript
import type { ApiVmCreateResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "networkName": null,
  "hypervisorSocket": null,
  "id": null,
  "nodeName": null,
} satisfies ApiVmCreateResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmCreateResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


