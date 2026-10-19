# ApiBillingGenerateResponseBillsInner


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**machines** | **List[str]** |  | 
**total_usd_micro** | **int** |  | 

## Example

```python
from jilebi_sdk.models.api_billing_generate_response_bills_inner import ApiBillingGenerateResponseBillsInner

# TODO update the JSON string below
json = "{}"
# create an instance of ApiBillingGenerateResponseBillsInner from a JSON string
api_billing_generate_response_bills_inner_instance = ApiBillingGenerateResponseBillsInner.from_json(json)
# print the JSON string representation of the object
print(ApiBillingGenerateResponseBillsInner.to_json())

# convert the object into a dict
api_billing_generate_response_bills_inner_dict = api_billing_generate_response_bills_inner_instance.to_dict()
# create an instance of ApiBillingGenerateResponseBillsInner from a dict
api_billing_generate_response_bills_inner_from_dict = ApiBillingGenerateResponseBillsInner.from_dict(api_billing_generate_response_bills_inner_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


