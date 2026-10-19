
# ApiBillingGenerateResponse


## Properties

Name | Type
------------ | -------------
`username` | string
`start` | number
`end` | number
`machinesChecked` | number
`billsCreated` | number
`emailsSent` | number
`bills` | [Array&lt;ApiBillingGenerateResponseBillsInner&gt;](ApiBillingGenerateResponseBillsInner.md)

## Example

```typescript
import type { ApiBillingGenerateResponse } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "username": null,
  "start": null,
  "end": null,
  "machinesChecked": null,
  "billsCreated": null,
  "emailsSent": null,
  "bills": null,
} satisfies ApiBillingGenerateResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiBillingGenerateResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


