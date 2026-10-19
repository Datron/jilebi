# ProvisionedVm

What provisioning a microVM on a node reports back.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_name** | **str** |  | 
**hypervisor_socket** | **str** |  | 

## Example

```python
from jilebi_sdk.models.provisioned_vm import ProvisionedVm

# TODO update the JSON string below
json = "{}"
# create an instance of ProvisionedVm from a JSON string
provisioned_vm_instance = ProvisionedVm.from_json(json)
# print the JSON string representation of the object
print(ProvisionedVm.to_json())

# convert the object into a dict
provisioned_vm_dict = provisioned_vm_instance.to_dict()
# create an instance of ProvisionedVm from a dict
provisioned_vm_from_dict = ProvisionedVm.from_dict(provisioned_vm_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


