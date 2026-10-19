# ApiSshKeyUpdateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **str** | New label for the key. Use an empty string to clear it. | [optional] 

## Example

```python
from jilebi_sdk.models.api_ssh_key_update_request import ApiSshKeyUpdateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiSshKeyUpdateRequest from a JSON string
api_ssh_key_update_request_instance = ApiSshKeyUpdateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiSshKeyUpdateRequest.to_json())

# convert the object into a dict
api_ssh_key_update_request_dict = api_ssh_key_update_request_instance.to_dict()
# create an instance of ApiSshKeyUpdateRequest from a dict
api_ssh_key_update_request_from_dict = ApiSshKeyUpdateRequest.from_dict(api_ssh_key_update_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


