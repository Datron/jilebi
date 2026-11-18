# ApiVmMetricsResponseCpu


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**usage_ratio** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metrics_response_cpu import ApiVmMetricsResponseCpu

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricsResponseCpu from a JSON string
api_vm_metrics_response_cpu_instance = ApiVmMetricsResponseCpu.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricsResponseCpu.to_json())

# convert the object into a dict
api_vm_metrics_response_cpu_dict = api_vm_metrics_response_cpu_instance.to_dict()
# create an instance of ApiVmMetricsResponseCpu from a dict
api_vm_metrics_response_cpu_from_dict = ApiVmMetricsResponseCpu.from_dict(api_vm_metrics_response_cpu_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


