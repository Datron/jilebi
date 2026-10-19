# ApiBillingGenerateResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **str** |  | 
**start** | **int** | Window start (epoch ms). | 
**end** | **int** | Window end (epoch ms). | 
**machines_checked** | **int** |  | 
**bills_created** | **int** |  | 
**emails_sent** | **int** |  | 
**bills** | [**List[ApiBillingGenerateResponseBillsInner]**](ApiBillingGenerateResponseBillsInner.md) |  | 

## Example

```python
from jilebi_sdk.models.api_billing_generate_response import ApiBillingGenerateResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiBillingGenerateResponse from a JSON string
api_billing_generate_response_instance = ApiBillingGenerateResponse.from_json(json)
# print the JSON string representation of the object
print(ApiBillingGenerateResponse.to_json())

# convert the object into a dict
api_billing_generate_response_dict = api_billing_generate_response_instance.to_dict()
# create an instance of ApiBillingGenerateResponse from a dict
api_billing_generate_response_from_dict = ApiBillingGenerateResponse.from_dict(api_billing_generate_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


