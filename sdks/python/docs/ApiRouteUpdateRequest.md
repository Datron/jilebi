# ApiRouteUpdateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**port** | **int** |  | [optional] 
**privacy** | **str** |  | [optional] 
**emails** | **List[str]** |  | [optional] 

## Example

```python
from jilebi_sdk.models.api_route_update_request import ApiRouteUpdateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiRouteUpdateRequest from a JSON string
api_route_update_request_instance = ApiRouteUpdateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiRouteUpdateRequest.to_json())

# convert the object into a dict
api_route_update_request_dict = api_route_update_request_instance.to_dict()
# create an instance of ApiRouteUpdateRequest from a dict
api_route_update_request_from_dict = ApiRouteUpdateRequest.from_dict(api_route_update_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


