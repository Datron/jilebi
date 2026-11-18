
# ApiVmMetricsSeries


## Properties

Name | Type
------------ | -------------
`points` | [Array&lt;ApiVmMetricPoint&gt;](ApiVmMetricPoint.md)
`current` | number
`average` | number
`max` | number

## Example

```typescript
import type { ApiVmMetricsSeries } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "points": null,
  "current": null,
  "average": null,
  "max": null,
} satisfies ApiVmMetricsSeries

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmMetricsSeries
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


