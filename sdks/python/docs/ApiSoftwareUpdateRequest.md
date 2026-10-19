# ApiSoftwareUpdateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **str** |  | [optional] 
**display_name** | **str** |  | [optional] 
**description** | **str** |  | [optional] 
**icon** | **str** |  | [optional] 
**install_command** | **str** |  | [optional] 
**setup_command** | **str** |  | [optional] 
**run_as_user** | **bool** |  | [optional] 
**recommended_vcpu** | **int** |  | [optional] 
**recommended_memory** | **int** |  | [optional] 
**recommended_disk** | **int** |  | [optional] 
**category** | **str** |  | [optional] 

## Example

```python
from jilebi_sdk.models.api_software_update_request import ApiSoftwareUpdateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiSoftwareUpdateRequest from a JSON string
api_software_update_request_instance = ApiSoftwareUpdateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiSoftwareUpdateRequest.to_json())

# convert the object into a dict
api_software_update_request_dict = api_software_update_request_instance.to_dict()
# create an instance of ApiSoftwareUpdateRequest from a dict
api_software_update_request_from_dict = ApiSoftwareUpdateRequest.from_dict(api_software_update_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


