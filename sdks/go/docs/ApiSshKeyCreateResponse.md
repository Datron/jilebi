# ApiSshKeyCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  | 
**Type** | **string** |  | 
**PublicKey** | **string** |  | 
**Fingerprint** | **string** |  | 
**PrivateKey** | Pointer to **string** | Only returned for generated keys. | [optional] 

## Methods

### NewApiSshKeyCreateResponse

`func NewApiSshKeyCreateResponse(id string, type_ string, publicKey string, fingerprint string, ) *ApiSshKeyCreateResponse`

NewApiSshKeyCreateResponse instantiates a new ApiSshKeyCreateResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiSshKeyCreateResponseWithDefaults

`func NewApiSshKeyCreateResponseWithDefaults() *ApiSshKeyCreateResponse`

NewApiSshKeyCreateResponseWithDefaults instantiates a new ApiSshKeyCreateResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ApiSshKeyCreateResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ApiSshKeyCreateResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ApiSshKeyCreateResponse) SetId(v string)`

SetId sets Id field to given value.


### GetType

`func (o *ApiSshKeyCreateResponse) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *ApiSshKeyCreateResponse) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *ApiSshKeyCreateResponse) SetType(v string)`

SetType sets Type field to given value.


### GetPublicKey

`func (o *ApiSshKeyCreateResponse) GetPublicKey() string`

GetPublicKey returns the PublicKey field if non-nil, zero value otherwise.

### GetPublicKeyOk

`func (o *ApiSshKeyCreateResponse) GetPublicKeyOk() (*string, bool)`

GetPublicKeyOk returns a tuple with the PublicKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicKey

`func (o *ApiSshKeyCreateResponse) SetPublicKey(v string)`

SetPublicKey sets PublicKey field to given value.


### GetFingerprint

`func (o *ApiSshKeyCreateResponse) GetFingerprint() string`

GetFingerprint returns the Fingerprint field if non-nil, zero value otherwise.

### GetFingerprintOk

`func (o *ApiSshKeyCreateResponse) GetFingerprintOk() (*string, bool)`

GetFingerprintOk returns a tuple with the Fingerprint field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFingerprint

`func (o *ApiSshKeyCreateResponse) SetFingerprint(v string)`

SetFingerprint sets Fingerprint field to given value.


### GetPrivateKey

`func (o *ApiSshKeyCreateResponse) GetPrivateKey() string`

GetPrivateKey returns the PrivateKey field if non-nil, zero value otherwise.

### GetPrivateKeyOk

`func (o *ApiSshKeyCreateResponse) GetPrivateKeyOk() (*string, bool)`

GetPrivateKeyOk returns a tuple with the PrivateKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrivateKey

`func (o *ApiSshKeyCreateResponse) SetPrivateKey(v string)`

SetPrivateKey sets PrivateKey field to given value.

### HasPrivateKey

`func (o *ApiSshKeyCreateResponse) HasPrivateKey() bool`

HasPrivateKey returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


