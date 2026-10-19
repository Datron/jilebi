# ApiKeyCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | **string** |  | 
**Scopes** | **[]string** |  | 
**ExpiresIn** | Pointer to [**ApiKeyCreateRequestExpiresIn**](ApiKeyCreateRequestExpiresIn.md) |  | [optional] 

## Methods

### NewApiKeyCreateRequest

`func NewApiKeyCreateRequest(name string, scopes []string, ) *ApiKeyCreateRequest`

NewApiKeyCreateRequest instantiates a new ApiKeyCreateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiKeyCreateRequestWithDefaults

`func NewApiKeyCreateRequestWithDefaults() *ApiKeyCreateRequest`

NewApiKeyCreateRequestWithDefaults instantiates a new ApiKeyCreateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *ApiKeyCreateRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiKeyCreateRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiKeyCreateRequest) SetName(v string)`

SetName sets Name field to given value.


### GetScopes

`func (o *ApiKeyCreateRequest) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *ApiKeyCreateRequest) GetScopesOk() (*[]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *ApiKeyCreateRequest) SetScopes(v []string)`

SetScopes sets Scopes field to given value.


### GetExpiresIn

`func (o *ApiKeyCreateRequest) GetExpiresIn() ApiKeyCreateRequestExpiresIn`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *ApiKeyCreateRequest) GetExpiresInOk() (*ApiKeyCreateRequestExpiresIn, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *ApiKeyCreateRequest) SetExpiresIn(v ApiKeyCreateRequestExpiresIn)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *ApiKeyCreateRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


