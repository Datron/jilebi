# ApiKey

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

## Methods

### NewApiKey

`func NewApiKey(id string, name NullableString, start NullableString, prefix NullableString, enabled bool, expiresAt NullableTime, createdAt NullableTime, lastRequest NullableTime, scopes []string, ) *ApiKey`

NewApiKey instantiates a new ApiKey object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiKeyWithDefaults

`func NewApiKeyWithDefaults() *ApiKey`

NewApiKeyWithDefaults instantiates a new ApiKey object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiKey) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiKey) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiKey) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ApiKey) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiKey) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiKey) SetName(v string)`

SetName sets Name field to given value.


### SetNameNil

`func (o *ApiKey) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *ApiKey) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetStart

`func (o *ApiKey) GetStart() string`

GetStart returns the Start field if non-nil, zero value otherwise.

### GetStartOk

`func (o *ApiKey) GetStartOk() (*string, bool)`

GetStartOk returns a tuple with the Start field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStart

`func (o *ApiKey) SetStart(v string)`

SetStart sets Start field to given value.


### SetStartNil

`func (o *ApiKey) SetStartNil(b bool)`

 SetStartNil sets the value for Start to be an explicit nil

### UnsetStart
`func (o *ApiKey) UnsetStart()`

UnsetStart ensures that no value is present for Start, not even an explicit nil
### GetPrefix

`func (o *ApiKey) GetPrefix() string`

GetPrefix returns the Prefix field if non-nil, zero value otherwise.

### GetPrefixOk

`func (o *ApiKey) GetPrefixOk() (*string, bool)`

GetPrefixOk returns a tuple with the Prefix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrefix

`func (o *ApiKey) SetPrefix(v string)`

SetPrefix sets Prefix field to given value.


### SetPrefixNil

`func (o *ApiKey) SetPrefixNil(b bool)`

 SetPrefixNil sets the value for Prefix to be an explicit nil

### UnsetPrefix
`func (o *ApiKey) UnsetPrefix()`

UnsetPrefix ensures that no value is present for Prefix, not even an explicit nil
### GetEnabled

`func (o *ApiKey) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *ApiKey) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *ApiKey) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetExpiresAt

`func (o *ApiKey) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *ApiKey) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *ApiKey) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.


### SetExpiresAtNil

`func (o *ApiKey) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *ApiKey) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetCreatedAt

`func (o *ApiKey) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiKey) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiKey) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### SetCreatedAtNil

`func (o *ApiKey) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *ApiKey) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetLastRequest

`func (o *ApiKey) GetLastRequest() time.Time`

GetLastRequest returns the LastRequest field if non-nil, zero value otherwise.

### GetLastRequestOk

`func (o *ApiKey) GetLastRequestOk() (*time.Time, bool)`

GetLastRequestOk returns a tuple with the LastRequest field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastRequest

`func (o *ApiKey) SetLastRequest(v time.Time)`

SetLastRequest sets LastRequest field to given value.


### SetLastRequestNil

`func (o *ApiKey) SetLastRequestNil(b bool)`

 SetLastRequestNil sets the value for LastRequest to be an explicit nil

### UnsetLastRequest
`func (o *ApiKey) UnsetLastRequest()`

UnsetLastRequest ensures that no value is present for LastRequest, not even an explicit nil
### GetScopes

`func (o *ApiKey) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *ApiKey) GetScopesOk() (*[]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *ApiKey) SetScopes(v []string)`

SetScopes sets Scopes field to given value.


### GetVia

`func (o *ApiKey) GetVia() string`

GetVia returns the Via field if non-nil, zero value otherwise.

### GetViaOk

`func (o *ApiKey) GetViaOk() (*string, bool)`

GetViaOk returns a tuple with the Via field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVia

`func (o *ApiKey) SetVia(v string)`

SetVia sets Via field to given value.

### HasVia

`func (o *ApiKey) HasVia() bool`

HasVia returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


