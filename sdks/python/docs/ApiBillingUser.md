# ApiBillingUser


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **str** |  | 
**name** | **str** |  | 
**email** | **str** |  | 

## Example

```python
from jilebi_sdk.models.api_billing_user import ApiBillingUser

# TODO update the JSON string below
json = "{}"
# create an instance of ApiBillingUser from a JSON string
api_billing_user_instance = ApiBillingUser.from_json(json)
# print the JSON string representation of the object
print(ApiBillingUser.to_json())

# convert the object into a dict
api_billing_user_dict = api_billing_user_instance.to_dict()
# create an instance of ApiBillingUser from a dict
api_billing_user_from_dict = ApiBillingUser.from_dict(api_billing_user_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


