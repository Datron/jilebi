# ApiNodeMaintenanceEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Username** | **string** |  | 
**MachineName** | **string** |  | 
**IpAddress** | Pointer to **NullableString** |  | [optional] 
**PrevState** | **string** | State the VM had before the run, and is restored to. | 
**State** | **string** | Snapshot progress. | 
**Restored** | **bool** | True once the VM has been woken back. | 
**DestinationUrl** | Pointer to **string** | Snapshot slot the VM was saved to. | [optional] 
**Error** | Pointer to **string** |  | [optional] 

## Methods

### NewApiNodeMaintenanceEntry

`func NewApiNodeMaintenanceEntry(username string, machineName string, prevState string, state string, restored bool, ) *ApiNodeMaintenanceEntry`

NewApiNodeMaintenanceEntry instantiates a new ApiNodeMaintenanceEntry object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiNodeMaintenanceEntryWithDefaults

`func NewApiNodeMaintenanceEntryWithDefaults() *ApiNodeMaintenanceEntry`

NewApiNodeMaintenanceEntryWithDefaults instantiates a new ApiNodeMaintenanceEntry object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetUsername

`func (o *ApiNodeMaintenanceEntry) GetUsername() string`

GetUsername returns the Username field if non-nil, zero value otherwise.

### GetUsernameOk

`func (o *ApiNodeMaintenanceEntry) GetUsernameOk() (*string, bool)`

GetUsernameOk returns a tuple with the Username field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsername

`func (o *ApiNodeMaintenanceEntry) SetUsername(v string)`

SetUsername sets Username field to given value.


### GetMachineName

`func (o *ApiNodeMaintenanceEntry) GetMachineName() string`

GetMachineName returns the MachineName field if non-nil, zero value otherwise.

### GetMachineNameOk

`func (o *ApiNodeMaintenanceEntry) GetMachineNameOk() (*string, bool)`

GetMachineNameOk returns a tuple with the MachineName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMachineName

`func (o *ApiNodeMaintenanceEntry) SetMachineName(v string)`

SetMachineName sets MachineName field to given value.


### GetIpAddress

`func (o *ApiNodeMaintenanceEntry) GetIpAddress() string`

GetIpAddress returns the IpAddress field if non-nil, zero value otherwise.

### GetIpAddressOk

`func (o *ApiNodeMaintenanceEntry) GetIpAddressOk() (*string, bool)`

GetIpAddressOk returns a tuple with the IpAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIpAddress

`func (o *ApiNodeMaintenanceEntry) SetIpAddress(v string)`

SetIpAddress sets IpAddress field to given value.

### HasIpAddress

`func (o *ApiNodeMaintenanceEntry) HasIpAddress() bool`

HasIpAddress returns a boolean if a field has been set.

### SetIpAddressNil

`func (o *ApiNodeMaintenanceEntry) SetIpAddressNil(b bool)`

 SetIpAddressNil sets the value for IpAddress to be an explicit nil

### UnsetIpAddress
`func (o *ApiNodeMaintenanceEntry) UnsetIpAddress()`

UnsetIpAddress ensures that no value is present for IpAddress, not even an explicit nil
### GetPrevState

`func (o *ApiNodeMaintenanceEntry) GetPrevState() string`

GetPrevState returns the PrevState field if non-nil, zero value otherwise.

### GetPrevStateOk

`func (o *ApiNodeMaintenanceEntry) GetPrevStateOk() (*string, bool)`

GetPrevStateOk returns a tuple with the PrevState field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrevState

`func (o *ApiNodeMaintenanceEntry) SetPrevState(v string)`

SetPrevState sets PrevState field to given value.


### GetState

`func (o *ApiNodeMaintenanceEntry) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *ApiNodeMaintenanceEntry) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *ApiNodeMaintenanceEntry) SetState(v string)`

SetState sets State field to given value.


### GetRestored

`func (o *ApiNodeMaintenanceEntry) GetRestored() bool`

GetRestored returns the Restored field if non-nil, zero value otherwise.

### GetRestoredOk

`func (o *ApiNodeMaintenanceEntry) GetRestoredOk() (*bool, bool)`

GetRestoredOk returns a tuple with the Restored field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRestored

`func (o *ApiNodeMaintenanceEntry) SetRestored(v bool)`

SetRestored sets Restored field to given value.


### GetDestinationUrl

`func (o *ApiNodeMaintenanceEntry) GetDestinationUrl() string`

GetDestinationUrl returns the DestinationUrl field if non-nil, zero value otherwise.

### GetDestinationUrlOk

`func (o *ApiNodeMaintenanceEntry) GetDestinationUrlOk() (*string, bool)`

GetDestinationUrlOk returns a tuple with the DestinationUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDestinationUrl

`func (o *ApiNodeMaintenanceEntry) SetDestinationUrl(v string)`

SetDestinationUrl sets DestinationUrl field to given value.

### HasDestinationUrl

`func (o *ApiNodeMaintenanceEntry) HasDestinationUrl() bool`

HasDestinationUrl returns a boolean if a field has been set.

### GetError

`func (o *ApiNodeMaintenanceEntry) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *ApiNodeMaintenanceEntry) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *ApiNodeMaintenanceEntry) SetError(v string)`

SetError sets Error field to given value.

### HasError

`func (o *ApiNodeMaintenanceEntry) HasError() bool`

HasError returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


