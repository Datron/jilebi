# ApiVmRestoreResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_url** | **str** |  | 
**new_name** | **str** | Present when the restore created a new microVM. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_restore_response import ApiVmRestoreResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmRestoreResponse from a JSON string
api_vm_restore_response_instance = ApiVmRestoreResponse.from_json(json)
# print the JSON string representation of the object
print(ApiVmRestoreResponse.to_json())

# convert the object into a dict
api_vm_restore_response_dict = api_vm_restore_response_instance.to_dict()
# create an instance of ApiVmRestoreResponse from a dict
api_vm_restore_response_from_dict = ApiVmRestoreResponse.from_dict(api_vm_restore_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


