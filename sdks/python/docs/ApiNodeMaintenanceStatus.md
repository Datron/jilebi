# ApiNodeMaintenanceStatus


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node** | **str** |  | 
**phase** | **str** |  | 
**running** | **bool** | True while the run is working in the background. | 
**started_at** | **datetime** |  | [optional] 
**routes_pending** | **bool** | True when every VM is back but some app routes still need replaying. | 
**entries** | [**List[ApiNodeMaintenanceEntry]**](ApiNodeMaintenanceEntry.md) |  | 

## Example

```python
from jilebi_sdk.models.api_node_maintenance_status import ApiNodeMaintenanceStatus

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNodeMaintenanceStatus from a JSON string
api_node_maintenance_status_instance = ApiNodeMaintenanceStatus.from_json(json)
# print the JSON string representation of the object
print(ApiNodeMaintenanceStatus.to_json())

# convert the object into a dict
api_node_maintenance_status_dict = api_node_maintenance_status_instance.to_dict()
# create an instance of ApiNodeMaintenanceStatus from a dict
api_node_maintenance_status_from_dict = ApiNodeMaintenanceStatus.from_dict(api_node_maintenance_status_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


