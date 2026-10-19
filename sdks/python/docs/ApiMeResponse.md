# ApiMeResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | **Dict[str, object]** | The authenticated user. | 
**session** | **Dict[str, object]** | The active session. | 

## Example

```python
from jilebi_sdk.models.api_me_response import ApiMeResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiMeResponse from a JSON string
api_me_response_instance = ApiMeResponse.from_json(json)
# print the JSON string representation of the object
print(ApiMeResponse.to_json())

# convert the object into a dict
api_me_response_dict = api_me_response_instance.to_dict()
# create an instance of ApiMeResponse from a dict
api_me_response_from_dict = ApiMeResponse.from_dict(api_me_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


