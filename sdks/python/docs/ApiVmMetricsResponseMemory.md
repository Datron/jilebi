# ApiVmMetricsResponseMemory


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**used_mb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 
**configured_mb** | [**ApiVmMetricsSeries**](ApiVmMetricsSeries.md) |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metrics_response_memory import ApiVmMetricsResponseMemory

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricsResponseMemory from a JSON string
api_vm_metrics_response_memory_instance = ApiVmMetricsResponseMemory.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricsResponseMemory.to_json())

# convert the object into a dict
api_vm_metrics_response_memory_dict = api_vm_metrics_response_memory_instance.to_dict()
# create an instance of ApiVmMetricsResponseMemory from a dict
api_vm_metrics_response_memory_from_dict = ApiVmMetricsResponseMemory.from_dict(api_vm_metrics_response_memory_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


