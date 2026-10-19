
# ApiVmRestoreRequest


## Properties

Name | Type
------------ | -------------
`sourceUrl` | string
`prefault` | boolean
`memoryRestoreMode` | string
`newName` | string

## Example

```typescript
import type { ApiVmRestoreRequest } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "sourceUrl": null,
  "prefault": null,
  "memoryRestoreMode": null,
  "newName": null,
} satisfies ApiVmRestoreRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmRestoreRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


