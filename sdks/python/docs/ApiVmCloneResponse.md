# ApiVmCloneResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_name** | **str** |  | 
**hypervisor_socket** | **str** |  | 
**id** | **str** |  | 
**node_name** | **str** |  | 
**new_name** | **str** |  | 

## Example

```python
from jilebi_sdk.models.api_vm_clone_response import ApiVmCloneResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmCloneResponse from a JSON string
api_vm_clone_response_instance = ApiVmCloneResponse.from_json(json)
# print the JSON string representation of the object
print(ApiVmCloneResponse.to_json())

# convert the object into a dict
api_vm_clone_response_dict = api_vm_clone_response_instance.to_dict()
# create an instance of ApiVmCloneResponse from a dict
api_vm_clone_response_from_dict = ApiVmCloneResponse.from_dict(api_vm_clone_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


