# ApiSshKeyCreateResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**type** | **str** |  | 
**public_key** | **str** |  | 
**fingerprint** | **str** |  | 
**private_key** | **str** | Only returned for generated keys. | [optional] 

## Example

```python
from jilebi_sdk.models.api_ssh_key_create_response import ApiSshKeyCreateResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiSshKeyCreateResponse from a JSON string
api_ssh_key_create_response_instance = ApiSshKeyCreateResponse.from_json(json)
# print the JSON string representation of the object
print(ApiSshKeyCreateResponse.to_json())

# convert the object into a dict
api_ssh_key_create_response_dict = api_ssh_key_create_response_instance.to_dict()
# create an instance of ApiSshKeyCreateResponse from a dict
api_ssh_key_create_response_from_dict = ApiSshKeyCreateResponse.from_dict(api_ssh_key_create_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


