
# ApiRouteUpdateRequest


## Properties

Name | Type
------------ | -------------
`port` | number
`privacy` | string
`emails` | Array&lt;string&gt;

## Example

```typescript
import type { ApiRouteUpdateRequest } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "port": null,
  "privacy": null,
  "emails": null,
} satisfies ApiRouteUpdateRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiRouteUpdateRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


