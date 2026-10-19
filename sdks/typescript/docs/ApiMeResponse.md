
# ApiMeResponse


## Properties

Name | Type
------------ | -------------
`user` | { [key: string]: any; }
`session` | { [key: string]: any; }

## Example

```typescript
import type { ApiMeResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "user": null,
  "session": null,
} satisfies ApiMeResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiMeResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


