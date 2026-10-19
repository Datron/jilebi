# ApiNodeRefreshResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **str** |  | 
**node** | [**ApiNode**](ApiNode.md) |  | 

## Example

```python
from jilebi_sdk.models.api_node_refresh_response import ApiNodeRefreshResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNodeRefreshResponse from a JSON string
api_node_refresh_response_instance = ApiNodeRefreshResponse.from_json(json)
# print the JSON string representation of the object
print(ApiNodeRefreshResponse.to_json())

# convert the object into a dict
api_node_refresh_response_dict = api_node_refresh_response_instance.to_dict()
# create an instance of ApiNodeRefreshResponse from a dict
api_node_refresh_response_from_dict = ApiNodeRefreshResponse.from_dict(api_node_refresh_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


