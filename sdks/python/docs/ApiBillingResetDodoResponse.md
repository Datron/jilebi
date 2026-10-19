# ApiBillingResetDodoResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **str** |  | 
**reset** | **bool** |  | 

## Example

```python
from jilebi_sdk.models.api_billing_reset_dodo_response import ApiBillingResetDodoResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiBillingResetDodoResponse from a JSON string
api_billing_reset_dodo_response_instance = ApiBillingResetDodoResponse.from_json(json)
# print the JSON string representation of the object
print(ApiBillingResetDodoResponse.to_json())

# convert the object into a dict
api_billing_reset_dodo_response_dict = api_billing_reset_dodo_response_instance.to_dict()
# create an instance of ApiBillingResetDodoResponse from a dict
api_billing_reset_dodo_response_from_dict = ApiBillingResetDodoResponse.from_dict(api_billing_reset_dodo_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


