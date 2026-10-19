# ApiBillingUsersResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**users** | [**List[ApiBillingUser]**](ApiBillingUser.md) |  | 

## Example

```python
from jilebi_sdk.models.api_billing_users_response import ApiBillingUsersResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiBillingUsersResponse from a JSON string
api_billing_users_response_instance = ApiBillingUsersResponse.from_json(json)
# print the JSON string representation of the object
print(ApiBillingUsersResponse.to_json())

# convert the object into a dict
api_billing_users_response_dict = api_billing_users_response_instance.to_dict()
# create an instance of ApiBillingUsersResponse from a dict
api_billing_users_response_from_dict = ApiBillingUsersResponse.from_dict(api_billing_users_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


