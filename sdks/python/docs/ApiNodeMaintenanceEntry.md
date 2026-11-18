# ApiNodeMaintenanceEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | **str** |  | 
**machine_name** | **str** |  | 
**ip_address** | **str** |  | [optional] 
**prev_state** | **str** | State the VM had before the run, and is restored to. | 
**state** | **str** | Snapshot progress. | 
**restored** | **bool** | True once the VM has been woken back. | 
**destination_url** | **str** | Snapshot slot the VM was saved to. | [optional] 
**error** | **str** |  | [optional] 

## Example

```python
from jilebi_sdk.models.api_node_maintenance_entry import ApiNodeMaintenanceEntry

# TODO update the JSON string below
json = "{}"
# create an instance of ApiNodeMaintenanceEntry from a JSON string
api_node_maintenance_entry_instance = ApiNodeMaintenanceEntry.from_json(json)
# print the JSON string representation of the object
print(ApiNodeMaintenanceEntry.to_json())

# convert the object into a dict
api_node_maintenance_entry_dict = api_node_maintenance_entry_instance.to_dict()
# create an instance of ApiNodeMaintenanceEntry from a dict
api_node_maintenance_entry_from_dict = ApiNodeMaintenanceEntry.from_dict(api_node_maintenance_entry_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


