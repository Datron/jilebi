# ApiSoftwareCreateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**name** | **str** |  | 
**display_name** | **str** |  | 
**description** | **str** |  | [optional] 
**icon** | **str** |  | [optional] 
**install_command** | **str** |  | 
**setup_command** | **str** |  | [optional] 
**run_as_user** | **bool** |  | [optional] 
**recommended_vcpu** | **int** |  | [optional] 
**recommended_memory** | **int** |  | [optional] 
**recommended_disk** | **int** |  | [optional] 
**category** | **str** |  | [optional] 

## Example

```python
from jilebi_sdk.models.api_software_create_request import ApiSoftwareCreateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiSoftwareCreateRequest from a JSON string
api_software_create_request_instance = ApiSoftwareCreateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiSoftwareCreateRequest.to_json())

# convert the object into a dict
api_software_create_request_dict = api_software_create_request_instance.to_dict()
# create an instance of ApiSoftwareCreateRequest from a dict
api_software_create_request_from_dict = ApiSoftwareCreateRequest.from_dict(api_software_create_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


