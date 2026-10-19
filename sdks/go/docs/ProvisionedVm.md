# ProvisionedVm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**NetworkName** | **string** |  | 
**HypervisorSocket** | **string** |  | 

## Methods

### NewProvisionedVm

`func NewProvisionedVm(networkName string, hypervisorSocket string, ) *ProvisionedVm`

NewProvisionedVm instantiates a new ProvisionedVm object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProvisionedVmWithDefaults

`func NewProvisionedVmWithDefaults() *ProvisionedVm`

NewProvisionedVmWithDefaults instantiates a new ProvisionedVm object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNetworkName

`func (o *ProvisionedVm) GetNetworkName() string`

GetNetworkName returns the NetworkName field if non-nil, zero value otherwise.

### GetNetworkNameOk

`func (o *ProvisionedVm) GetNetworkNameOk() (*string, bool)`

GetNetworkNameOk returns a tuple with the NetworkName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNetworkName

`func (o *ProvisionedVm) SetNetworkName(v string)`

SetNetworkName sets NetworkName field to given value.


### GetHypervisorSocket

`func (o *ProvisionedVm) GetHypervisorSocket() string`

GetHypervisorSocket returns the HypervisorSocket field if non-nil, zero value otherwise.

### GetHypervisorSocketOk

`func (o *ProvisionedVm) GetHypervisorSocketOk() (*string, bool)`

GetHypervisorSocketOk returns a tuple with the HypervisorSocket field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHypervisorSocket

`func (o *ProvisionedVm) SetHypervisorSocket(v string)`

SetHypervisorSocket sets HypervisorSocket field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


