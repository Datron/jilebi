
# ApiKey


## Properties

Name | Type
------------ | -------------
`id` | string
`name` | string
`start` | string
`prefix` | string
`enabled` | boolean
`expiresAt` | Date
`createdAt` | Date
`lastRequest` | Date
`scopes` | Array&lt;string&gt;
`via` | string

## Example

```typescript
import type { ApiKey } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "name": null,
  "start": null,
  "prefix": null,
  "enabled": null,
  "expiresAt": null,
  "createdAt": null,
  "lastRequest": null,
  "scopes": null,
  "via": null,
} satisfies ApiKey

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiKey
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


