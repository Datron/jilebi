
# ApiRoute


## Properties

Name | Type
------------ | -------------
`id` | string
`name` | string
`microvmId` | string
`privacy` | string
`port` | number
`userIds` | Array&lt;string&gt;
`machineName` | string
`machineUserId` | string
`nodeName` | string
`ipAddress` | string
`ownerUsername` | string
`ownerEmail` | string
`createdAt` | Date
`updatedAt` | Date
`url` | string
`sharedUsers` | [Array&lt;SharedUser&gt;](SharedUser.md)

## Example

```typescript
import type { ApiRoute } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "name": null,
  "microvmId": null,
  "privacy": null,
  "port": null,
  "userIds": null,
  "machineName": null,
  "machineUserId": null,
  "nodeName": null,
  "ipAddress": null,
  "ownerUsername": null,
  "ownerEmail": null,
  "createdAt": null,
  "updatedAt": null,
  "url": null,
  "sharedUsers": null,
} satisfies ApiRoute

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiRoute
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


