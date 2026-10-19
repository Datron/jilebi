# ApiKey


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**name** | **str** |  | 
**start** | **str** |  | 
**prefix** | **str** |  | 
**enabled** | **bool** |  | 
**expires_at** | **datetime** |  | 
**created_at** | **datetime** |  | 
**last_request** | **datetime** |  | 
**scopes** | **List[str]** |  | 
**via** | **str** | Set when the key was minted by sshoxy. | [optional] 

## Example

```python
from jilebi_sdk.models.api_key import ApiKey

# TODO update the JSON string below
json = "{}"
# create an instance of ApiKey from a JSON string
api_key_instance = ApiKey.from_json(json)
# print the JSON string representation of the object
print(ApiKey.to_json())

# convert the object into a dict
api_key_dict = api_key_instance.to_dict()
# create an instance of ApiKey from a dict
api_key_from_dict = ApiKey.from_dict(api_key_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


