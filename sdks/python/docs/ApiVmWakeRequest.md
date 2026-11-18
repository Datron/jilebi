# ApiVmWakeRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ip_address** | **str** | Guest IP; only needed to recreate the tap device if it is missing. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_wake_request import ApiVmWakeRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmWakeRequest from a JSON string
api_vm_wake_request_instance = ApiVmWakeRequest.from_json(json)
# print the JSON string representation of the object
print(ApiVmWakeRequest.to_json())

# convert the object into a dict
api_vm_wake_request_dict = api_vm_wake_request_instance.to_dict()
# create an instance of ApiVmWakeRequest from a dict
api_vm_wake_request_from_dict = ApiVmWakeRequest.from_dict(api_vm_wake_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


