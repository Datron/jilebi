# SharedUser


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**email** | **str** |  | 
**name** | **str** |  | 

## Example

```python
from jilebi_sdk.models.shared_user import SharedUser

# TODO update the JSON string below
json = "{}"
# create an instance of SharedUser from a JSON string
shared_user_instance = SharedUser.from_json(json)
# print the JSON string representation of the object
print(SharedUser.to_json())

# convert the object into a dict
shared_user_dict = shared_user_instance.to_dict()
# create an instance of SharedUser from a dict
shared_user_from_dict = SharedUser.from_dict(shared_user_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


