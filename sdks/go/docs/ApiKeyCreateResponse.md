# ApiKeyCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Name** | **NullableString** |  | 
**Start** | **NullableString** |  | 
**Prefix** | **NullableString** |  | 
**Enabled** | **bool** |  | 
**ExpiresAt** | **NullableTime** |  | 
**CreatedAt** | **NullableTime** |  | 
**LastRequest** | **NullableTime** |  | 
**Scopes** | **[]string** |  | 
**Via** | Pointer to **string** | Set when the key was minted by sshoxy. | [optional] 
**Key** | **string** | Shown once. Store it securely. | 

## Methods

### NewApiKeyCreateResponse

`func NewApiKeyCreateResponse(id string, name NullableString, start NullableString, prefix NullableString, enabled bool, expiresAt NullableTime, createdAt NullableTime, lastRequest NullableTime, scopes []string, key string, ) *ApiKeyCreateResponse`

NewApiKeyCreateResponse instantiates a new ApiKeyCreateResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiKeyCreateResponseWithDefaults

`func NewApiKeyCreateResponseWithDefaults() *ApiKeyCreateResponse`

NewApiKeyCreateResponseWithDefaults instantiates a new ApiKeyCreateResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiKeyCreateResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiKeyCreateResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiKeyCreateResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ApiKeyCreateResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiKeyCreateResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiKeyCreateResponse) SetName(v string)`

SetName sets Name field to given value.


### SetNameNil

`func (o *ApiKeyCreateResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *ApiKeyCreateResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetStart

`func (o *ApiKeyCreateResponse) GetStart() string`

GetStart returns the Start field if non-nil, zero value otherwise.

### GetStartOk

`func (o *ApiKeyCreateResponse) GetStartOk() (*string, bool)`

GetStartOk returns a tuple with the Start field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStart

`func (o *ApiKeyCreateResponse) SetStart(v string)`

SetStart sets Start field to given value.


### SetStartNil

`func (o *ApiKeyCreateResponse) SetStartNil(b bool)`

 SetStartNil sets the value for Start to be an explicit nil

### UnsetStart
`func (o *ApiKeyCreateResponse) UnsetStart()`

UnsetStart ensures that no value is present for Start, not even an explicit nil
### GetPrefix

`func (o *ApiKeyCreateResponse) GetPrefix() string`

GetPrefix returns the Prefix field if non-nil, zero value otherwise.

### GetPrefixOk

`func (o *ApiKeyCreateResponse) GetPrefixOk() (*string, bool)`

GetPrefixOk returns a tuple with the Prefix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrefix

`func (o *ApiKeyCreateResponse) SetPrefix(v string)`

SetPrefix sets Prefix field to given value.


### SetPrefixNil

`func (o *ApiKeyCreateResponse) SetPrefixNil(b bool)`

 SetPrefixNil sets the value for Prefix to be an explicit nil

### UnsetPrefix
`func (o *ApiKeyCreateResponse) UnsetPrefix()`

UnsetPrefix ensures that no value is present for Prefix, not even an explicit nil
### GetEnabled

`func (o *ApiKeyCreateResponse) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *ApiKeyCreateResponse) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *ApiKeyCreateResponse) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetExpiresAt

`func (o *ApiKeyCreateResponse) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *ApiKeyCreateResponse) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *ApiKeyCreateResponse) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.


### SetExpiresAtNil

`func (o *ApiKeyCreateResponse) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *ApiKeyCreateResponse) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetCreatedAt

`func (o *ApiKeyCreateResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiKeyCreateResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiKeyCreateResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### SetCreatedAtNil

`func (o *ApiKeyCreateResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *ApiKeyCreateResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetLastRequest

`func (o *ApiKeyCreateResponse) GetLastRequest() time.Time`

GetLastRequest returns the LastRequest field if non-nil, zero value otherwise.

### GetLastRequestOk

`func (o *ApiKeyCreateResponse) GetLastRequestOk() (*time.Time, bool)`

GetLastRequestOk returns a tuple with the LastRequest field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastRequest

`func (o *ApiKeyCreateResponse) SetLastRequest(v time.Time)`

SetLastRequest sets LastRequest field to given value.


### SetLastRequestNil

`func (o *ApiKeyCreateResponse) SetLastRequestNil(b bool)`

 SetLastRequestNil sets the value for LastRequest to be an explicit nil

### UnsetLastRequest
`func (o *ApiKeyCreateResponse) UnsetLastRequest()`

UnsetLastRequest ensures that no value is present for LastRequest, not even an explicit nil
### GetScopes

`func (o *ApiKeyCreateResponse) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *ApiKeyCreateResponse) GetScopesOk() (*[]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *ApiKeyCreateResponse) SetScopes(v []string)`

SetScopes sets Scopes field to given value.


### GetVia

`func (o *ApiKeyCreateResponse) GetVia() string`

GetVia returns the Via field if non-nil, zero value otherwise.

### GetViaOk

`func (o *ApiKeyCreateResponse) GetViaOk() (*string, bool)`

GetViaOk returns a tuple with the Via field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVia

`func (o *ApiKeyCreateResponse) SetVia(v string)`

SetVia sets Via field to given value.

### HasVia

`func (o *ApiKeyCreateResponse) HasVia() bool`

HasVia returns a boolean if a field has been set.

### GetKey

`func (o *ApiKeyCreateResponse) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *ApiKeyCreateResponse) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *ApiKeyCreateResponse) SetKey(v string)`

SetKey sets Key field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


