# ApiVmSnapshotResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DestinationUrl** | **string** |  | 
**Resumed** | **bool** | Whether the VM was resumed after snapshotting. | 

## Methods

### NewApiVmSnapshotResponse

`func NewApiVmSnapshotResponse(destinationUrl string, resumed bool, ) *ApiVmSnapshotResponse`

NewApiVmSnapshotResponse instantiates a new ApiVmSnapshotResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmSnapshotResponseWithDefaults

`func NewApiVmSnapshotResponseWithDefaults() *ApiVmSnapshotResponse`

NewApiVmSnapshotResponseWithDefaults instantiates a new ApiVmSnapshotResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDestinationUrl

`func (o *ApiVmSnapshotResponse) GetDestinationUrl() string`

GetDestinationUrl returns the DestinationUrl field if non-nil, zero value otherwise.

### GetDestinationUrlOk

`func (o *ApiVmSnapshotResponse) GetDestinationUrlOk() (*string, bool)`

GetDestinationUrlOk returns a tuple with the DestinationUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDestinationUrl

`func (o *ApiVmSnapshotResponse) SetDestinationUrl(v string)`

SetDestinationUrl sets DestinationUrl field to given value.


### GetResumed

`func (o *ApiVmSnapshotResponse) GetResumed() bool`

GetResumed returns the Resumed field if non-nil, zero value otherwise.

### GetResumedOk

`func (o *ApiVmSnapshotResponse) GetResumedOk() (*bool, bool)`

GetResumedOk returns a tuple with the Resumed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResumed

`func (o *ApiVmSnapshotResponse) SetResumed(v bool)`

SetResumed sets Resumed field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


