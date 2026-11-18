# ApiVmMetricPoint


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**t** | **float** | Unix epoch seconds (may include fractional seconds). | 
**v** | **float** |  | 

## Example

```python
from jilebi_sdk.models.api_vm_metric_point import ApiVmMetricPoint

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmMetricPoint from a JSON string
api_vm_metric_point_instance = ApiVmMetricPoint.from_json(json)
# print the JSON string representation of the object
print(ApiVmMetricPoint.to_json())

# convert the object into a dict
api_vm_metric_point_dict = api_vm_metric_point_instance.to_dict()
# create an instance of ApiVmMetricPoint from a dict
api_vm_metric_point_from_dict = ApiVmMetricPoint.from_dict(api_vm_metric_point_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


