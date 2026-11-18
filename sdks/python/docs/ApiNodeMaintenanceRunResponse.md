# ApiNodeMaintenanceRunResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node** | **str** |  | 
**phase** | **str** |  | 
**running** | **bool** | True while the run is working in the background. | 
**started_at** | **datetime** |  | [optional] 
**routes_pending** | **bool** | True when every VM is back but some app routes still need replaying. | 
**entries** | [**List[ApiNodeMaintenanceEntry]**](ApiNodeMaintenanceEntry.md) |  | 
**message** | **str** | Human summary of the run. | 

## Example

```python
from jilebi_sdk.models.api_node_maintenance_run_response import ApiNodeMaintenanceRunResponse

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNodeMaintenanceRunResponse from a JSON string
api_node_maintenance_run_response_instance = ApiNodeMaintenanceRunResponse.from_json(json)
# print the JSON string representation of the object
print(ApiNodeMaintenanceRunResponse.to_json())

# convert the object into a dict
api_node_maintenance_run_response_dict = api_node_maintenance_run_response_instance.to_dict()
# create an instance of ApiNodeMaintenanceRunResponse from a dict
api_node_maintenance_run_response_from_dict = ApiNodeMaintenanceRunResponse.from_dict(api_node_maintenance_run_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


