# ApiVmCloneRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**new_name** | **str** |  | 
**public_key** | **str** | Defaults to the caller&#39;s saved SSH key. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_clone_request import ApiVmCloneRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmCloneRequest from a JSON string
api_vm_clone_request_instance = ApiVmCloneRequest.from_json(json)
# print the JSON string representation of the object
print(ApiVmCloneRequest.to_json())

# convert the object into a dict
api_vm_clone_request_dict = api_vm_clone_request_instance.to_dict()
# create an instance of ApiVmCloneRequest from a dict
api_vm_clone_request_from_dict = ApiVmCloneRequest.from_dict(api_vm_clone_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


