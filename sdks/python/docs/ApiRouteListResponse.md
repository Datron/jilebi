# ApiRouteListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owned** | [**List[ApiRoute]**](ApiRoute.md) |  | 
**shared** | [**List[ApiRoute]**](ApiRoute.md) |  | 

## Example

```python
from jilebi_sdk.models.api_route_list_response import ApiRouteListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiRouteListResponse from a JSON string
api_route_list_response_instance = ApiRouteListResponse.from_json(json)
# print the JSON string representation of the object
print(ApiRouteListResponse.to_json())

# convert the object into a dict
api_route_list_response_dict = api_route_list_response_instance.to_dict()
# create an instance of ApiRouteListResponse from a dict
api_route_list_response_from_dict = ApiRouteListResponse.from_dict(api_route_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


