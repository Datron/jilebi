
# ApiVmMetricsResponseMemory


## Properties

Name | Type
------------ | -------------
`usedMb` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)
`configuredMb` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)

## Example

```typescript
import type { ApiVmMetricsResponseMemory } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "usedMb": null,
  "configuredMb": null,
} satisfies ApiVmMetricsResponseMemory

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmMetricsResponseMemory
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


