# ApiRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**microvm_id** | **String** |  | 
**privacy** | **Privacy** |  (enum: private, public, shared) | 
**port** | **i32** |  | 
**user_ids** | **Vec<String>** |  | 
**machine_name** | **String** |  | 
**machine_user_id** | **String** |  | 
**node_name** | **String** |  | 
**ip_address** | Option<**String**> |  | 
**owner_username** | Option<**String**> |  | 
**owner_email** | **String** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**url** | **String** |  | 
**shared_users** | [**Vec<models::SharedUser>**](SharedUser.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


