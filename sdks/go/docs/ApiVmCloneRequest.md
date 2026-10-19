# ApiVmCloneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**NewName** | **string** |  | 
**PublicKey** | Pointer to **string** | Defaults to the caller&#39;s saved SSH key. | [optional] 

## Methods

### NewApiVmCloneRequest

`func NewApiVmCloneRequest(newName string, ) *ApiVmCloneRequest`

NewApiVmCloneRequest instantiates a new ApiVmCloneRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmCloneRequestWithDefaults

`func NewApiVmCloneRequestWithDefaults() *ApiVmCloneRequest`

NewApiVmCloneRequestWithDefaults instantiates a new ApiVmCloneRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNewName

`func (o *ApiVmCloneRequest) GetNewName() string`

GetNewName returns the NewName field if non-nil, zero value otherwise.

### GetNewNameOk

`func (o *ApiVmCloneRequest) GetNewNameOk() (*string, bool)`

GetNewNameOk returns a tuple with the NewName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNewName

`func (o *ApiVmCloneRequest) SetNewName(v string)`

SetNewName sets NewName field to given value.


### GetPublicKey

`func (o *ApiVmCloneRequest) GetPublicKey() string`

GetPublicKey returns the PublicKey field if non-nil, zero value otherwise.

### GetPublicKeyOk

`func (o *ApiVmCloneRequest) GetPublicKeyOk() (*string, bool)`

GetPublicKeyOk returns a tuple with the PublicKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicKey

`func (o *ApiVmCloneRequest) SetPublicKey(v string)`

SetPublicKey sets PublicKey field to given value.

### HasPublicKey

`func (o *ApiVmCloneRequest) HasPublicKey() bool`

HasPublicKey returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


