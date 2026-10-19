
# ApiRouteListResponse


## Properties

Name | Type
------------ | -------------
`owned` | [Array&lt;ApiRoute&gt;](ApiRoute.md)
`shared` | [Array&lt;ApiRoute&gt;](ApiRoute.md)

## Example

```typescript
import type { ApiRouteListResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "owned": null,
  "shared": null,
} satisfies ApiRouteListResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiRouteListResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


