# ApiVm


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**user_id** | **str** |  | 
**machine_name** | **str** |  | 
**network_name** | **str** |  | 
**node_name** | **str** |  | 
**hypervisor_socket** | **str** |  | 
**ip_address** | **str** | Private network address (e.g. 192.168.2.2). | 
**cpu_count** | **int** | vCPU count. | 
**memory_size** | **int** | Memory in MiB. | 
**disk_size_gb** | **int** | Disk size in GiB. | 
**state** | [**ApiVmState**](ApiVmState.md) |  | 
**created_at** | **datetime** |  | 
**updated_at** | **datetime** |  | 

## Example

```python
from jilebi_sdk.models.api_vm import ApiVm

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVm from a JSON string
api_vm_instance = ApiVm.from_json(json)
# print the JSON string representation of the object
print(ApiVm.to_json())

# convert the object into a dict
api_vm_dict = api_vm_instance.to_dict()
# create an instance of ApiVm from a dict
api_vm_from_dict = ApiVm.from_dict(api_vm_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


