
# ApiVmMetricsResponseNetwork


## Properties

Name | Type
------------ | -------------
`rxBps` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)
`txBps` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)

## Example

```typescript
import type { ApiVmMetricsResponseNetwork } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "rxBps": null,
  "txBps": null,
} satisfies ApiVmMetricsResponseNetwork

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmMetricsResponseNetwork
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


