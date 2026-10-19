
# ApiSoftwareUpdateRequest


## Properties

Name | Type
------------ | -------------
`name` | string
`displayName` | string
`description` | string
`icon` | string
`installCommand` | string
`setupCommand` | string
`runAsUser` | boolean
`recommendedVcpu` | number
`recommendedMemory` | number
`recommendedDisk` | number
`category` | string

## Example

```typescript
import type { ApiSoftwareUpdateRequest } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "name": null,
  "displayName": null,
  "description": null,
  "icon": null,
  "installCommand": null,
  "setupCommand": null,
  "runAsUser": null,
  "recommendedVcpu": null,
  "recommendedMemory": null,
  "recommendedDisk": null,
  "category": null,
} satisfies ApiSoftwareUpdateRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiSoftwareUpdateRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


