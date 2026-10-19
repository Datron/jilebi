# ApiSshKey


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**name** | **str** |  | 
**public_key** | **str** |  | 
**fingerprint** | **str** |  | 
**type** | **str** | &#x60;ed25519&#x60; for generated keys, &#x60;uploaded&#x60; otherwise. | 
**created_at** | **datetime** |  | 

## Example

```python
from jilebi_sdk.models.api_ssh_key import ApiSshKey

# TODO update the JSON string below
json = "{}"
# create an instance of ApiSshKey from a JSON string
api_ssh_key_instance = ApiSshKey.from_json(json)
# print the JSON string representation of the object
print(ApiSshKey.to_json())

# convert the object into a dict
api_ssh_key_dict = api_ssh_key_instance.to_dict()
# create an instance of ApiSshKey from a dict
api_ssh_key_from_dict = ApiSshKey.from_dict(api_ssh_key_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


