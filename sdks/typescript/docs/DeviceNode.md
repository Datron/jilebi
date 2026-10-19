
# DeviceNode

Node in the hypervisor device tree.

## Properties

Name | Type
------------ | -------------
`id` | string
`resources` | Array&lt;any&gt;
`children` | Array&lt;string&gt;
`pciBdf` | string

## Example

```typescript
import type { DeviceNode } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "resources": null,
  "children": null,
  "pciBdf": null,
} satisfies DeviceNode

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as DeviceNode
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


