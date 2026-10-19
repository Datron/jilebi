# ApiSshKeyCreateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**type** | **str** |  | 
**name** | **str** |  | [optional] 
**public_key** | **str** | Required when &#x60;type&#x60; is &#x60;upload&#x60;. | [optional] 

## Example

```python
from jilebi_sdk.models.api_ssh_key_create_request import ApiSshKeyCreateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiSshKeyCreateRequest from a JSON string
api_ssh_key_create_request_instance = ApiSshKeyCreateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiSshKeyCreateRequest.to_json())

# convert the object into a dict
api_ssh_key_create_request_dict = api_ssh_key_create_request_instance.to_dict()
# create an instance of ApiSshKeyCreateRequest from a dict
api_ssh_key_create_request_from_dict = ApiSshKeyCreateRequest.from_dict(api_ssh_key_create_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


