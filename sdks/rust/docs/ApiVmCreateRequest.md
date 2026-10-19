# ApiVmCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloudinit_user_data** | **Vec<String>** | cloud-init `runcmd` entries, de-duplicated with the selected software's commands. | 
**software_ids** | Option<**Vec<String>**> | Software catalog ids to install. | [optional]
**public_key** | **String** | SSH public key injected into the VM. | 
**cpu_count** | **i32** |  | 
**memory_size** | **i32** | Memory in MiB. | 
**disk_size_gb** | **i32** | Disk size in GiB. | 
**cloning** | Option<**String**> | Existing machine name to clone from. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


