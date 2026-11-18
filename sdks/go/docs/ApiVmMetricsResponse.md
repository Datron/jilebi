# ApiVmMetricsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MachineName** | **string** |  | 
**NodeName** | **string** |  | 
**Range** | **string** |  | 
**Start** | **int32** | Window start, Unix epoch seconds. | 
**End** | **int32** | Window end, Unix epoch seconds. | 
**StepSeconds** | **int32** |  | 
**Available** | **bool** | True when at least one metrics query succeeded. | 
**Error** | Pointer to **string** | Why metrics could not be read. | [optional] 
**Configured** | [**ApiVmMetricsResponseConfigured**](ApiVmMetricsResponseConfigured.md) |  | 
**Cpu** | [**ApiVmMetricsResponseCpu**](ApiVmMetricsResponseCpu.md) |  | 
**Memory** | [**ApiVmMetricsResponseMemory**](ApiVmMetricsResponseMemory.md) |  | 
**Network** | [**ApiVmMetricsResponseNetwork**](ApiVmMetricsResponseNetwork.md) |  | 
**Disk** | [**ApiVmMetricsResponseDisk**](ApiVmMetricsResponseDisk.md) |  | 

## Methods

### NewApiVmMetricsResponse

`func NewApiVmMetricsResponse(machineName string, nodeName string, range_ string, start int32, end int32, stepSeconds int32, available bool, configured ApiVmMetricsResponseConfigured, cpu ApiVmMetricsResponseCpu, memory ApiVmMetricsResponseMemory, network ApiVmMetricsResponseNetwork, disk ApiVmMetricsResponseDisk, ) *ApiVmMetricsResponse`

NewApiVmMetricsResponse instantiates a new ApiVmMetricsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmMetricsResponseWithDefaults

`func NewApiVmMetricsResponseWithDefaults() *ApiVmMetricsResponse`

NewApiVmMetricsResponseWithDefaults instantiates a new ApiVmMetricsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMachineName

`func (o *ApiVmMetricsResponse) GetMachineName() string`

GetMachineName returns the MachineName field if non-nil, zero value otherwise.

### GetMachineNameOk

`func (o *ApiVmMetricsResponse) GetMachineNameOk() (*string, bool)`

GetMachineNameOk returns a tuple with the MachineName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMachineName

`func (o *ApiVmMetricsResponse) SetMachineName(v string)`

SetMachineName sets MachineName field to given value.


### GetNodeName

`func (o *ApiVmMetricsResponse) GetNodeName() string`

GetNodeName returns the NodeName field if non-nil, zero value otherwise.

### GetNodeNameOk

`func (o *ApiVmMetricsResponse) GetNodeNameOk() (*string, bool)`

GetNodeNameOk returns a tuple with the NodeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNodeName

`func (o *ApiVmMetricsResponse) SetNodeName(v string)`

SetNodeName sets NodeName field to given value.


### GetRange

`func (o *ApiVmMetricsResponse) GetRange() string`

GetRange returns the Range field if non-nil, zero value otherwise.

### GetRangeOk

`func (o *ApiVmMetricsResponse) GetRangeOk() (*string, bool)`

GetRangeOk returns a tuple with the Range field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRange

`func (o *ApiVmMetricsResponse) SetRange(v string)`

SetRange sets Range field to given value.


### GetStart

`func (o *ApiVmMetricsResponse) GetStart() int32`

GetStart returns the Start field if non-nil, zero value otherwise.

### GetStartOk

`func (o *ApiVmMetricsResponse) GetStartOk() (*int32, bool)`

GetStartOk returns a tuple with the Start field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStart

`func (o *ApiVmMetricsResponse) SetStart(v int32)`

SetStart sets Start field to given value.


### GetEnd

`func (o *ApiVmMetricsResponse) GetEnd() int32`

GetEnd returns the End field if non-nil, zero value otherwise.

### GetEndOk

`func (o *ApiVmMetricsResponse) GetEndOk() (*int32, bool)`

GetEndOk returns a tuple with the End field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnd

`func (o *ApiVmMetricsResponse) SetEnd(v int32)`

SetEnd sets End field to given value.


### GetStepSeconds

`func (o *ApiVmMetricsResponse) GetStepSeconds() int32`

GetStepSeconds returns the StepSeconds field if non-nil, zero value otherwise.

### GetStepSecondsOk

`func (o *ApiVmMetricsResponse) GetStepSecondsOk() (*int32, bool)`

GetStepSecondsOk returns a tuple with the StepSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStepSeconds

`func (o *ApiVmMetricsResponse) SetStepSeconds(v int32)`

SetStepSeconds sets StepSeconds field to given value.


### GetAvailable

`func (o *ApiVmMetricsResponse) GetAvailable() bool`

GetAvailable returns the Available field if non-nil, zero value otherwise.

### GetAvailableOk

`func (o *ApiVmMetricsResponse) GetAvailableOk() (*bool, bool)`

GetAvailableOk returns a tuple with the Available field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAvailable

`func (o *ApiVmMetricsResponse) SetAvailable(v bool)`

SetAvailable sets Available field to given value.


### GetError

`func (o *ApiVmMetricsResponse) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *ApiVmMetricsResponse) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *ApiVmMetricsResponse) SetError(v string)`

SetError sets Error field to given value.

### HasError

`func (o *ApiVmMetricsResponse) HasError() bool`

HasError returns a boolean if a field has been set.

### GetConfigured

`func (o *ApiVmMetricsResponse) GetConfigured() ApiVmMetricsResponseConfigured`

GetConfigured returns the Configured field if non-nil, zero value otherwise.

### GetConfiguredOk

`func (o *ApiVmMetricsResponse) GetConfiguredOk() (*ApiVmMetricsResponseConfigured, bool)`

GetConfiguredOk returns a tuple with the Configured field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfigured

`func (o *ApiVmMetricsResponse) SetConfigured(v ApiVmMetricsResponseConfigured)`

SetConfigured sets Configured field to given value.


### GetCpu

`func (o *ApiVmMetricsResponse) GetCpu() ApiVmMetricsResponseCpu`

GetCpu returns the Cpu field if non-nil, zero value otherwise.

### GetCpuOk

`func (o *ApiVmMetricsResponse) GetCpuOk() (*ApiVmMetricsResponseCpu, bool)`

GetCpuOk returns a tuple with the Cpu field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCpu

`func (o *ApiVmMetricsResponse) SetCpu(v ApiVmMetricsResponseCpu)`

SetCpu sets Cpu field to given value.


### GetMemory

`func (o *ApiVmMetricsResponse) GetMemory() ApiVmMetricsResponseMemory`

GetMemory returns the Memory field if non-nil, zero value otherwise.

### GetMemoryOk

`func (o *ApiVmMetricsResponse) GetMemoryOk() (*ApiVmMetricsResponseMemory, bool)`

GetMemoryOk returns a tuple with the Memory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemory

`func (o *ApiVmMetricsResponse) SetMemory(v ApiVmMetricsResponseMemory)`

SetMemory sets Memory field to given value.


### GetNetwork

`func (o *ApiVmMetricsResponse) GetNetwork() ApiVmMetricsResponseNetwork`

GetNetwork returns the Network field if non-nil, zero value otherwise.

### GetNetworkOk

`func (o *ApiVmMetricsResponse) GetNetworkOk() (*ApiVmMetricsResponseNetwork, bool)`

GetNetworkOk returns a tuple with the Network field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNetwork

`func (o *ApiVmMetricsResponse) SetNetwork(v ApiVmMetricsResponseNetwork)`

SetNetwork sets Network field to given value.


### GetDisk

`func (o *ApiVmMetricsResponse) GetDisk() ApiVmMetricsResponseDisk`

GetDisk returns the Disk field if non-nil, zero value otherwise.

### GetDiskOk

`func (o *ApiVmMetricsResponse) GetDiskOk() (*ApiVmMetricsResponseDisk, bool)`

GetDiskOk returns a tuple with the Disk field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisk

`func (o *ApiVmMetricsResponse) SetDisk(v ApiVmMetricsResponseDisk)`

SetDisk sets Disk field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


