# ApiRoute


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**name** | **str** |  | 
**microvm_id** | **str** |  | 
**privacy** | **str** |  | 
**port** | **int** |  | 
**user_ids** | **List[str]** |  | 
**machine_name** | **str** |  | 
**machine_user_id** | **str** |  | 
**node_name** | **str** |  | 
**ip_address** | **str** |  | 
**owner_username** | **str** |  | 
**owner_email** | **str** |  | 
**created_at** | **datetime** |  | 
**updated_at** | **datetime** |  | 
**url** | **str** |  | 
**shared_users** | [**List[SharedUser]**](SharedUser.md) |  | 

## Example

```python
from jilebi_sdk.models.api_route import ApiRoute

# TODO update the JSON string below
json = "{}"
# create an instance of ApiRoute from a JSON string
api_route_instance = ApiRoute.from_json(json)
# print the JSON string representation of the object
print(ApiRoute.to_json())

# convert the object into a dict
api_route_dict = api_route_instance.to_dict()
# create an instance of ApiRoute from a dict
api_route_from_dict = ApiRoute.from_dict(api_route_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


