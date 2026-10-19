
# ApiKeyCreateRequest


## Properties

Name | Type
------------ | -------------
`name` | string
`scopes` | Array&lt;string&gt;
`expiresIn` | [ApiKeyCreateRequestExpiresIn](ApiKeyCreateRequestExpiresIn.md)

## Example

```typescript
import type { ApiKeyCreateRequest } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "name": null,
  "scopes": null,
  "expiresIn": null,
} satisfies ApiKeyCreateRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiKeyCreateRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


