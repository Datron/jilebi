
# ApiNodeUpdateRequest


## Properties

Name | Type
------------ | -------------
`specialization` | string
`status` | [ApiNodeStatus](ApiNodeStatus.md)

## Example

```typescript
import type { ApiNodeUpdateRequest } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "specialization": null,
  "status": null,
} satisfies ApiNodeUpdateRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiNodeUpdateRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


