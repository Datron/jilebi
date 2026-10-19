# ApiKeyCreateRequestExpiresIn

Lifetime: `never`, `7d`, `30d`, `90d`, or a positive number of seconds.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------

## Example

```python
from jilebi_sdk.models.api_key_create_request_expires_in import ApiKeyCreateRequestExpiresIn

# TODO update the JSON string below
json = "{}"
# create an instance of ApiKeyCreateRequestExpiresIn from a JSON string
api_key_create_request_expires_in_instance = ApiKeyCreateRequestExpiresIn.from_json(json)
# print the JSON string representation of the object
print(ApiKeyCreateRequestExpiresIn.to_json())

# convert the object into a dict
api_key_create_request_expires_in_dict = api_key_create_request_expires_in_instance.to_dict()
# create an instance of ApiKeyCreateRequestExpiresIn from a dict
api_key_create_request_expires_in_from_dict = ApiKeyCreateRequestExpiresIn.from_dict(api_key_create_request_expires_in_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


