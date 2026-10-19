# ApiNodeUpdateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**specialization** | **str** |  | [optional] 
**status** | [**ApiNodeStatus**](ApiNodeStatus.md) |  | [optional] 

## Example

```python
from jilebi_sdk.models.api_node_update_request import ApiNodeUpdateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNodeUpdateRequest from a JSON string
api_node_update_request_instance = ApiNodeUpdateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiNodeUpdateRequest.to_json())

# convert the object into a dict
api_node_update_request_dict = api_node_update_request_instance.to_dict()
# create an instance of ApiNodeUpdateRequest from a dict
api_node_update_request_from_dict = ApiNodeUpdateRequest.from_dict(api_node_update_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


