# ApiRouteCreateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **str** |  | 
**microvm_id** | **str** | Id of the VM the route forwards to. | 
**port** | **int** |  | 
**privacy** | **str** |  | 
**emails** | **List[str]** | Required when &#x60;privacy&#x60; is &#x60;shared&#x60;. | [optional] 

## Example

```python
from jilebi_sdk.models.api_route_create_request import ApiRouteCreateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiRouteCreateRequest from a JSON string
api_route_create_request_instance = ApiRouteCreateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiRouteCreateRequest.to_json())

# convert the object into a dict
api_route_create_request_dict = api_route_create_request_instance.to_dict()
# create an instance of ApiRouteCreateRequest from a dict
api_route_create_request_from_dict = ApiRouteCreateRequest.from_dict(api_route_create_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


