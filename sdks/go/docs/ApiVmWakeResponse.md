# ApiVmWakeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Success** | Pointer to **bool** |  | [optional] 
**SourceUrl** | Pointer to **NullableString** | Snapshot restored from, when not resumed in place. | [optional] 
**AlreadyRunning** | **bool** | True when the VM was already running. | 
**ResumedInPlace** | **bool** | True when a live-paused VM was resumed. | 
**ColdBooted** | **bool** | True when no snapshot was usable and the existing disk was booted fresh (RAM state lost). | 
**TookMs** | Pointer to **int32** | Time taken to bring the VM back. | [optional] 
**Deduped** | Pointer to **bool** | True when an identical wake was already running and this call was a no-op. | [optional] 
**StateChanged** | Pointer to **bool** | True when this call actually moved the VM to running. Callers reserve node capacity only when this is true. | [optional] 

## Methods

### NewApiVmWakeResponse

`func NewApiVmWakeResponse(alreadyRunning bool, resumedInPlace bool, coldBooted bool, ) *ApiVmWakeResponse`

NewApiVmWakeResponse instantiates a new ApiVmWakeResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmWakeResponseWithDefaults

`func NewApiVmWakeResponseWithDefaults() *ApiVmWakeResponse`

NewApiVmWakeResponseWithDefaults instantiates a new ApiVmWakeResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSuccess

`func (o *ApiVmWakeResponse) GetSuccess() bool`

GetSuccess returns the Success field if non-nil, zero value otherwise.

### GetSuccessOk

`func (o *ApiVmWakeResponse) GetSuccessOk() (*bool, bool)`

GetSuccessOk returns a tuple with the Success field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuccess

`func (o *ApiVmWakeResponse) SetSuccess(v bool)`

SetSuccess sets Success field to given value.

### HasSuccess

`func (o *ApiVmWakeResponse) HasSuccess() bool`

HasSuccess returns a boolean if a field has been set.

### GetSourceUrl

`func (o *ApiVmWakeResponse) GetSourceUrl() string`

GetSourceUrl returns the SourceUrl field if non-nil, zero value otherwise.

### GetSourceUrlOk

`func (o *ApiVmWakeResponse) GetSourceUrlOk() (*string, bool)`

GetSourceUrlOk returns a tuple with the SourceUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceUrl

`func (o *ApiVmWakeResponse) SetSourceUrl(v string)`

SetSourceUrl sets SourceUrl field to given value.

### HasSourceUrl

`func (o *ApiVmWakeResponse) HasSourceUrl() bool`

HasSourceUrl returns a boolean if a field has been set.

### SetSourceUrlNil

`func (o *ApiVmWakeResponse) SetSourceUrlNil(b bool)`

 SetSourceUrlNil sets the value for SourceUrl to be an explicit nil

### UnsetSourceUrl
`func (o *ApiVmWakeResponse) UnsetSourceUrl()`

UnsetSourceUrl ensures that no value is present for SourceUrl, not even an explicit nil
### GetAlreadyRunning

`func (o *ApiVmWakeResponse) GetAlreadyRunning() bool`

GetAlreadyRunning returns the AlreadyRunning field if non-nil, zero value otherwise.

### GetAlreadyRunningOk

`func (o *ApiVmWakeResponse) GetAlreadyRunningOk() (*bool, bool)`

GetAlreadyRunningOk returns a tuple with the AlreadyRunning field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAlreadyRunning

`func (o *ApiVmWakeResponse) SetAlreadyRunning(v bool)`

SetAlreadyRunning sets AlreadyRunning field to given value.


### GetResumedInPlace

`func (o *ApiVmWakeResponse) GetResumedInPlace() bool`

GetResumedInPlace returns the ResumedInPlace field if non-nil, zero value otherwise.

### GetResumedInPlaceOk

`func (o *ApiVmWakeResponse) GetResumedInPlaceOk() (*bool, bool)`

GetResumedInPlaceOk returns a tuple with the ResumedInPlace field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResumedInPlace

`func (o *ApiVmWakeResponse) SetResumedInPlace(v bool)`

SetResumedInPlace sets ResumedInPlace field to given value.


### GetColdBooted

`func (o *ApiVmWakeResponse) GetColdBooted() bool`

GetColdBooted returns the ColdBooted field if non-nil, zero value otherwise.

### GetColdBootedOk

`func (o *ApiVmWakeResponse) GetColdBootedOk() (*bool, bool)`

GetColdBootedOk returns a tuple with the ColdBooted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetColdBooted

`func (o *ApiVmWakeResponse) SetColdBooted(v bool)`

SetColdBooted sets ColdBooted field to given value.


### GetTookMs

`func (o *ApiVmWakeResponse) GetTookMs() int32`

GetTookMs returns the TookMs field if non-nil, zero value otherwise.

### GetTookMsOk

`func (o *ApiVmWakeResponse) GetTookMsOk() (*int32, bool)`

GetTookMsOk returns a tuple with the TookMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTookMs

`func (o *ApiVmWakeResponse) SetTookMs(v int32)`

SetTookMs sets TookMs field to given value.

### HasTookMs

`func (o *ApiVmWakeResponse) HasTookMs() bool`

HasTookMs returns a boolean if a field has been set.

### GetDeduped

`func (o *ApiVmWakeResponse) GetDeduped() bool`

GetDeduped returns the Deduped field if non-nil, zero value otherwise.

### GetDedupedOk

`func (o *ApiVmWakeResponse) GetDedupedOk() (*bool, bool)`

GetDedupedOk returns a tuple with the Deduped field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeduped

`func (o *ApiVmWakeResponse) SetDeduped(v bool)`

SetDeduped sets Deduped field to given value.

### HasDeduped

`func (o *ApiVmWakeResponse) HasDeduped() bool`

HasDeduped returns a boolean if a field has been set.

### GetStateChanged

`func (o *ApiVmWakeResponse) GetStateChanged() bool`

GetStateChanged returns the StateChanged field if non-nil, zero value otherwise.

### GetStateChangedOk

`func (o *ApiVmWakeResponse) GetStateChangedOk() (*bool, bool)`

GetStateChangedOk returns a tuple with the StateChanged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStateChanged

`func (o *ApiVmWakeResponse) SetStateChanged(v bool)`

SetStateChanged sets StateChanged field to given value.

### HasStateChanged

`func (o *ApiVmWakeResponse) HasStateChanged() bool`

HasStateChanged returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


