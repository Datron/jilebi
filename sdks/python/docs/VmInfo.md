# VmInfo

Live VM state as reported by the hypervisor.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | **Dict[str, object]** | Cloud-hypervisor VM config. | 
**state** | **str** |  | 
**memory_actual_size** | **int** | Memory in bytes. | [optional] 
**device_tree** | [**Dict[str, DeviceNode]**](DeviceNode.md) |  | [optional] 

## Example

```python
from jilebi_sdk.models.vm_info import VmInfo

# TODO update the JSON string below
json = "{}"
# create an instance of VmInfo from a JSON string
vm_info_instance = VmInfo.from_json(json)
# print the JSON string representation of the object
print(VmInfo.to_json())

# convert the object into a dict
vm_info_dict = vm_info_instance.to_dict()
# create an instance of VmInfo from a dict
vm_info_from_dict = VmInfo.from_dict(vm_info_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


