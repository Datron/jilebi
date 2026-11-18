# ApiVmMetricsSeries


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**points** | [**List[ApiVmMetricPoint]**](ApiVmMetricPoint.md) |  | 
**current** | **float** | Last sample in the window. | 
**average** | **float** |  | 
**max** | **float** |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metrics_series import ApiVmMetricsSeries

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricsSeries from a JSON string
api_vm_metrics_series_instance = ApiVmMetricsSeries.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricsSeries.to_json())

# convert the object into a dict
api_vm_metrics_series_dict = api_vm_metrics_series_instance.to_dict()
# create an instance of ApiVmMetricsSeries from a dict
api_vm_metrics_series_from_dict = ApiVmMetricsSeries.from_dict(api_vm_metrics_series_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


