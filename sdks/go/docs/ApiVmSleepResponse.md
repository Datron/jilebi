# ApiVmSleepResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Success** | Pointer to **bool** |  | [optional] 
**DestinationUrl** | **string** | Snapshot slot URL the VM was saved to. | 
**AlreadySleeping** | **bool** | True when the VM was already evicted. | 
**PauseMs** | Pointer to **int32** | Time spent pausing the vCPUs. | [optional] 
**SnapshotMs** | Pointer to **int32** | Time spent writing the snapshot. | [optional] 
**TotalMs** | Pointer to **int32** | Total time for the sleep operation. | [optional] 
**Deduped** | Pointer to **bool** | True when an identical sleep was already running and this call was a no-op. | [optional] 
**StateChanged** | Pointer to **bool** | True when this call actually put the VM to sleep. Callers release node capacity only when this is true. | [optional] 

## Methods

### NewApiVmSleepResponse

`func NewApiVmSleepResponse(destinationUrl string, alreadySleeping bool, ) *ApiVmSleepResponse`

NewApiVmSleepResponse instantiates a new ApiVmSleepResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmSleepResponseWithDefaults

`func NewApiVmSleepResponseWithDefaults() *ApiVmSleepResponse`

NewApiVmSleepResponseWithDefaults instantiates a new ApiVmSleepResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSuccess

`func (o *ApiVmSleepResponse) GetSuccess() bool`

GetSuccess returns the Success field if non-nil, zero value otherwise.

### GetSuccessOk

`func (o *ApiVmSleepResponse) GetSuccessOk() (*bool, bool)`

GetSuccessOk returns a tuple with the Success field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuccess

`func (o *ApiVmSleepResponse) SetSuccess(v bool)`

SetSuccess sets Success field to given value.

### HasSuccess

`func (o *ApiVmSleepResponse) HasSuccess() bool`

HasSuccess returns a boolean if a field has been set.

### GetDestinationUrl

`func (o *ApiVmSleepResponse) GetDestinationUrl() string`

GetDestinationUrl returns the DestinationUrl field if non-nil, zero value otherwise.

### GetDestinationUrlOk

`func (o *ApiVmSleepResponse) GetDestinationUrlOk() (*string, bool)`

GetDestinationUrlOk returns a tuple with the DestinationUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDestinationUrl

`func (o *ApiVmSleepResponse) SetDestinationUrl(v string)`

SetDestinationUrl sets DestinationUrl field to given value.


### GetAlreadySleeping

`func (o *ApiVmSleepResponse) GetAlreadySleeping() bool`

GetAlreadySleeping returns the AlreadySleeping field if non-nil, zero value otherwise.

### GetAlreadySleepingOk

`func (o *ApiVmSleepResponse) GetAlreadySleepingOk() (*bool, bool)`

GetAlreadySleepingOk returns a tuple with the AlreadySleeping field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAlreadySleeping

`func (o *ApiVmSleepResponse) SetAlreadySleeping(v bool)`

SetAlreadySleeping sets AlreadySleeping field to given value.


### GetPauseMs

`func (o *ApiVmSleepResponse) GetPauseMs() int32`

GetPauseMs returns the PauseMs field if non-nil, zero value otherwise.

### GetPauseMsOk

`func (o *ApiVmSleepResponse) GetPauseMsOk() (*int32, bool)`

GetPauseMsOk returns a tuple with the PauseMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPauseMs

`func (o *ApiVmSleepResponse) SetPauseMs(v int32)`

SetPauseMs sets PauseMs field to given value.

### HasPauseMs

`func (o *ApiVmSleepResponse) HasPauseMs() bool`

HasPauseMs returns a boolean if a field has been set.

### GetSnapshotMs

`func (o *ApiVmSleepResponse) GetSnapshotMs() int32`

GetSnapshotMs returns the SnapshotMs field if non-nil, zero value otherwise.

### GetSnapshotMsOk

`func (o *ApiVmSleepResponse) GetSnapshotMsOk() (*int32, bool)`

GetSnapshotMsOk returns a tuple with the SnapshotMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSnapshotMs

`func (o *ApiVmSleepResponse) SetSnapshotMs(v int32)`

SetSnapshotMs sets SnapshotMs field to given value.

### HasSnapshotMs

`func (o *ApiVmSleepResponse) HasSnapshotMs() bool`

HasSnapshotMs returns a boolean if a field has been set.

### GetTotalMs

`func (o *ApiVmSleepResponse) GetTotalMs() int32`

GetTotalMs returns the TotalMs field if non-nil, zero value otherwise.

### GetTotalMsOk

`func (o *ApiVmSleepResponse) GetTotalMsOk() (*int32, bool)`

GetTotalMsOk returns a tuple with the TotalMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalMs

`func (o *ApiVmSleepResponse) SetTotalMs(v int32)`

SetTotalMs sets TotalMs field to given value.

### HasTotalMs

`func (o *ApiVmSleepResponse) HasTotalMs() bool`

HasTotalMs returns a boolean if a field has been set.

### GetDeduped

`func (o *ApiVmSleepResponse) GetDeduped() bool`

GetDeduped returns the Deduped field if non-nil, zero value otherwise.

### GetDedupedOk

`func (o *ApiVmSleepResponse) GetDedupedOk() (*bool, bool)`

GetDedupedOk returns a tuple with the Deduped field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeduped

`func (o *ApiVmSleepResponse) SetDeduped(v bool)`

SetDeduped sets Deduped field to given value.

### HasDeduped

`func (o *ApiVmSleepResponse) HasDeduped() bool`

HasDeduped returns a boolean if a field has been set.

### GetStateChanged

`func (o *ApiVmSleepResponse) GetStateChanged() bool`

GetStateChanged returns the StateChanged field if non-nil, zero value otherwise.

### GetStateChangedOk

`func (o *ApiVmSleepResponse) GetStateChangedOk() (*bool, bool)`

GetStateChangedOk returns a tuple with the StateChanged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStateChanged

`func (o *ApiVmSleepResponse) SetStateChanged(v bool)`

SetStateChanged sets StateChanged field to given value.

### HasStateChanged

`func (o *ApiVmSleepResponse) HasStateChanged() bool`

HasStateChanged returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


