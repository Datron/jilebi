# ApiNodeCreateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **str** |  | 
**url** | **str** |  | 
**specialization** | **str** |  | [optional] 

## Example

```python
from jilebi_sdk.models.api_node_create_request import ApiNodeCreateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNodeCreateRequest from a JSON string
api_node_create_request_instance = ApiNodeCreateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiNodeCreateRequest.to_json())

# convert the object into a dict
api_node_create_request_dict = api_node_create_request_instance.to_dict()
# create an instance of ApiNodeCreateRequest from a dict
api_node_create_request_from_dict = ApiNodeCreateRequest.from_dict(api_node_create_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


