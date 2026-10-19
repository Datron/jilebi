# ApiVmCreateResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_name** | **str** |  | 
**hypervisor_socket** | **str** |  | 
**id** | **str** |  | 
**node_name** | **str** |  | 

## Example

```python
from jilebi_sdk.models.api_vm_create_response import ApiVmCreateResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmCreateResponse from a JSON string
api_vm_create_response_instance = ApiVmCreateResponse.from_json(json)
# print the JSON string representation of the object
print(ApiVmCreateResponse.to_json())

# convert the object into a dict
api_vm_create_response_dict = api_vm_create_response_instance.to_dict()
# create an instance of ApiVmCreateResponse from a dict
api_vm_create_response_from_dict = ApiVmCreateResponse.from_dict(api_vm_create_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


