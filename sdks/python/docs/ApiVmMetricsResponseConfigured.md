# ApiVmMetricsResponseConfigured


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_count** | **int** |  | 
**memory_mb** | **int** |  | 
**disk_gb** | **int** |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metrics_response_configured import ApiVmMetricsResponseConfigured

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricsResponseConfigured from a JSON string
api_vm_metrics_response_configured_instance = ApiVmMetricsResponseConfigured.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricsResponseConfigured.to_json())

# convert the object into a dict
api_vm_metrics_response_configured_dict = api_vm_metrics_response_configured_instance.to_dict()
# create an instance of ApiVmMetricsResponseConfigured from a dict
api_vm_metrics_response_configured_from_dict = ApiVmMetricsResponseConfigured.from_dict(api_vm_metrics_response_configured_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


