# ApiSshKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Name** | **NullableString** |  | 
**PublicKey** | **string** |  | 
**Fingerprint** | **NullableString** |  | 
**Type** | **NullableString** | &#x60;ed25519&#x60; for generated keys, &#x60;uploaded&#x60; otherwise. | 
**CreatedAt** | **time.Time** |  | 

## Methods

### NewApiSshKey

`func NewApiSshKey(id string, name NullableString, publicKey string, fingerprint NullableString, type_ NullableString, createdAt time.Time, ) *ApiSshKey`

NewApiSshKey instantiates a new ApiSshKey object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiSshKeyWithDefaults

`func NewApiSshKeyWithDefaults() *ApiSshKey`

NewApiSshKeyWithDefaults instantiates a new ApiSshKey object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiSshKey) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiSshKey) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiSshKey) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ApiSshKey) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ApiSshKey) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ApiSshKey) SetName(v string)`

SetName sets Name field to given value.


### SetNameNil

`func (o *ApiSshKey) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *ApiSshKey) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetPublicKey

`func (o *ApiSshKey) GetPublicKey() string`

GetPublicKey returns the PublicKey field if non-nil, zero value otherwise.

### GetPublicKeyOk

`func (o *ApiSshKey) GetPublicKeyOk() (*string, bool)`

GetPublicKeyOk returns a tuple with the PublicKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicKey

`func (o *ApiSshKey) SetPublicKey(v string)`

SetPublicKey sets PublicKey field to given value.


### GetFingerprint

`func (o *ApiSshKey) GetFingerprint() string`

GetFingerprint returns the Fingerprint field if non-nil, zero value otherwise.

### GetFingerprintOk

`func (o *ApiSshKey) GetFingerprintOk() (*string, bool)`

GetFingerprintOk returns a tuple with the Fingerprint field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFingerprint

`func (o *ApiSshKey) SetFingerprint(v string)`

SetFingerprint sets Fingerprint field to given value.


### SetFingerprintNil

`func (o *ApiSshKey) SetFingerprintNil(b bool)`

 SetFingerprintNil sets the value for Fingerprint to be an explicit nil

### UnsetFingerprint
`func (o *ApiSshKey) UnsetFingerprint()`

UnsetFingerprint ensures that no value is present for Fingerprint, not even an explicit nil
### GetType

`func (o *ApiSshKey) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *ApiSshKey) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *ApiSshKey) SetType(v string)`

SetType sets Type field to given value.


### SetTypeNil

`func (o *ApiSshKey) SetTypeNil(b bool)`

 SetTypeNil sets the value for Type to be an explicit nil

### UnsetType
`func (o *ApiSshKey) UnsetType()`

UnsetType ensures that no value is present for Type, not even an explicit nil
### GetCreatedAt

`func (o *ApiSshKey) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ApiSshKey) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ApiSshKey) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


