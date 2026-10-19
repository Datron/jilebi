# ApiVmRestoreRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_url** | **str** |  | 
**prefault** | **bool** |  | [optional] 
**memory_restore_mode** | **str** |  | [optional] 
**new_name** | **str** | Restore into a new microVM with this name. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_restore_request import ApiVmRestoreRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmRestoreRequest from a JSON string
api_vm_restore_request_instance = ApiVmRestoreRequest.from_json(json)
# print the JSON string representation of the object
print(ApiVmRestoreRequest.to_json())

# convert the object into a dict
api_vm_restore_request_dict = api_vm_restore_request_instance.to_dict()
# create an instance of ApiVmRestoreRequest from a dict
api_vm_restore_request_from_dict = ApiVmRestoreRequest.from_dict(api_vm_restore_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


