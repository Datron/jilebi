
# ApiVmCreateRequest


## Properties

Name | Type
------------ | -------------
`cloudinitUserData` | Array&lt;string&gt;
`softwareIds` | Array&lt;string&gt;
`publicKey` | string
`cpuCount` | number
`memorySize` | number
`diskSizeGb` | number
`cloning` | string

## Example

```typescript
import type { ApiVmCreateRequest } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "cloudinitUserData": null,
  "softwareIds": null,
  "publicKey": null,
  "cpuCount": null,
  "memorySize": null,
  "diskSizeGb": null,
  "cloning": null,
} satisfies ApiVmCreateRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmCreateRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


