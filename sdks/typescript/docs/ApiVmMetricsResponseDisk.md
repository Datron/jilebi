
# ApiVmMetricsResponseDisk


## Properties

Name | Type
------------ | -------------
`usedGb` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)
`configuredGb` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)
`readGbps` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)
`writeGbps` | [ApiVmMetricsSeries](ApiVmMetricsSeries.md)

## Example

```typescript
import type { ApiVmMetricsResponseDisk } from '@jilebi/sdk'

// TODO: Update the object below with actual values
const example = {
  "usedGb": null,
  "configuredGb": null,
  "readGbps": null,
  "writeGbps": null,
} satisfies ApiVmMetricsResponseDisk

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiVmMetricsResponseDisk
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


