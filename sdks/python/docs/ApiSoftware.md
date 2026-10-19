# ApiSoftware


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**name** | **str** |  | 
**display_name** | **str** |  | 
**description** | **str** |  | 
**icon** | **str** |  | 
**install_command** | **str** |  | 
**setup_command** | **str** |  | 
**run_as_user** | **bool** |  | 
**recommended_vcpu** | **int** |  | 
**recommended_memory** | **int** |  | 
**recommended_disk** | **int** |  | 
**category** | **str** |  | 
**created_at** | **datetime** |  | 
**updated_at** | **datetime** |  | 

## Example

```python
from jilebi_sdk.models.api_software import ApiSoftware

# TODO update the JSON string below
json = "{}"
# create an instance of ApiSoftware from a JSON string
api_software_instance = ApiSoftware.from_json(json)
# print the JSON string representation of the object
print(ApiSoftware.to_json())

# convert the object into a dict
api_software_dict = api_software_instance.to_dict()
# create an instance of ApiSoftware from a dict
api_software_from_dict = ApiSoftware.from_dict(api_software_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


