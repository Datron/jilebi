
# ApiRouteCreateResponse


## Properties

Name | Type
------------ | -------------
`id` | string
`name` | string
`microvmId` | string
`privacy` | string
`port` | number
`url` | string
`sharedUsers` | [Array&lt;SharedUser&gt;](SharedUser.md)

## Example

```typescript
import type { ApiRouteCreateResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "name": null,
  "microvmId": null,
  "privacy": null,
  "port": null,
  "url": null,
  "sharedUsers": null,
} satisfies ApiRouteCreateResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiRouteCreateResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


