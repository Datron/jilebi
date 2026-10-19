# DeviceNode

Node in the hypervisor device tree.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | [optional] 
**resources** | **List[object]** |  | [optional] 
**children** | **List[str]** |  | [optional] 
**pci_bdf** | **str** |  | [optional] 

## Example

```python
from jilebi_sdk.models.device_node import DeviceNode

# TODO update the JSON string below
json = "{}"
# create an instance of DeviceNode from a JSON string
device_node_instance = DeviceNode.from_json(json)
# print the JSON string representation of the object
print(DeviceNode.to_json())

# convert the object into a dict
device_node_dict = device_node_instance.to_dict()
# create an instance of DeviceNode from a dict
device_node_from_dict = DeviceNode.from_dict(device_node_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


