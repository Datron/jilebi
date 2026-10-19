# ApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | Option<**String**> |  | 
**start** | Option<**String**> |  | 
**prefix** | Option<**String**> |  | 
**enabled** | **bool** |  | 
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | 
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | 
**last_request** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | 
**scopes** | **Vec<Scopes>** |  (enum: vms:read, vms:write, routes:read, routes:write, nodes:read, nodes:write, software:read, software:write, billing:read, billing:write, keys:read, keys:write, api-keys:read, api-keys:write) | 
**via** | Option<**String**> | Set when the key was minted by sshoxy. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


