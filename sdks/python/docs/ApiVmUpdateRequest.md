# ApiVmUpdateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_count** | **int** |  | [optional] 
**memory_size** | **int** |  | [optional] 
**disk_size_gb** | **int** |  | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_update_request import ApiVmUpdateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmUpdateRequest from a JSON string
api_vm_update_request_instance = ApiVmUpdateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiVmUpdateRequest.to_json())

# convert the object into a dict
api_vm_update_request_dict = api_vm_update_request_instance.to_dict()
# create an instance of ApiVmUpdateRequest from a dict
api_vm_update_request_from_dict = ApiVmUpdateRequest.from_dict(api_vm_update_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


