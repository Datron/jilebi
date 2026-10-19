# ApiRouteCreateResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**name** | **str** |  | 
**microvm_id** | **str** |  | 
**privacy** | **str** |  | 
**port** | **int** |  | 
**url** | **str** |  | 
**shared_users** | [**List[SharedUser]**](SharedUser.md) |  | 

## Example

```python
from jilebi_sdk.models.api_route_create_response import ApiRouteCreateResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiRouteCreateResponse from a JSON string
api_route_create_response_instance = ApiRouteCreateResponse.from_json(json)
# print the JSON string representation of the object
print(ApiRouteCreateResponse.to_json())

# convert the object into a dict
api_route_create_response_dict = api_route_create_response_instance.to_dict()
# create an instance of ApiRouteCreateResponse from a dict
api_route_create_response_from_dict = ApiRouteCreateResponse.from_dict(api_route_create_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


