
# ApiSshKeyCreateResponse


## Properties

Name | Type
------------ | -------------
`id` | string
`type` | string
`publicKey` | string
`fingerprint` | string
`privateKey` | string

## Example

```typescript
import type { ApiSshKeyCreateResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "type": null,
  "publicKey": null,
  "fingerprint": null,
  "privateKey": null,
} satisfies ApiSshKeyCreateResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiSshKeyCreateResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


