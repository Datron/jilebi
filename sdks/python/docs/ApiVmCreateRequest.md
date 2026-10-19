# ApiVmCreateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloudinit_user_data** | **List[str]** | cloud-init &#x60;runcmd&#x60; entries, de-duplicated with the selected software&#39;s commands. | 
**software_ids** | **List[str]** | Software catalog ids to install. | [optional] 
**public_key** | **str** | SSH public key injected into the VM. | 
**cpu_count** | **int** |  | 
**memory_size** | **int** | Memory in MiB. | 
**disk_size_gb** | **int** | Disk size in GiB. | 
**cloning** | **str** | Existing machine name to clone from. | [optional] 

## Example

```python
from jilebi_sdk.models.api_vm_create_request import ApiVmCreateRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ApiVmCreateRequest from a JSON string
api_vm_create_request_instance = ApiVmCreateRequest.from_json(json)
# print the JSON string representation of the object
print(ApiVmCreateRequest.to_json())

# convert the object into a dict
api_vm_create_request_dict = api_vm_create_request_instance.to_dict()
# create an instance of ApiVmCreateRequest from a dict
api_vm_create_request_from_dict = ApiVmCreateRequest.from_dict(api_vm_create_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


