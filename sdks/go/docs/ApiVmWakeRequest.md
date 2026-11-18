# ApiVmWakeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IpAddress** | Pointer to **string** | Guest IP; only needed to recreate the tap device if it is missing. | [optional] 

## Methods

### NewApiVmWakeRequest

`func NewApiVmWakeRequest() *ApiVmWakeRequest`

NewApiVmWakeRequest instantiates a new ApiVmWakeRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmWakeRequestWithDefaults

`func NewApiVmWakeRequestWithDefaults() *ApiVmWakeRequest`

NewApiVmWakeRequestWithDefaults instantiates a new ApiVmWakeRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIpAddress

`func (o *ApiVmWakeRequest) GetIpAddress() string`

GetIpAddress returns the IpAddress field if non-nil, zero value otherwise.

### GetIpAddressOk

`func (o *ApiVmWakeRequest) GetIpAddressOk() (*string, bool)`

GetIpAddressOk returns a tuple with the IpAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIpAddress

`func (o *ApiVmWakeRequest) SetIpAddress(v string)`

SetIpAddress sets IpAddress field to given value.

### HasIpAddress

`func (o *ApiVmWakeRequest) HasIpAddress() bool`

HasIpAddress returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


