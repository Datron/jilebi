# ApiVmSnapshotRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DestinationUrl** | Pointer to **string** | Object-storage URL to upload the snapshot to. | [optional] 

## Methods

### NewApiVmSnapshotRequest

`func NewApiVmSnapshotRequest() *ApiVmSnapshotRequest`

NewApiVmSnapshotRequest instantiates a new ApiVmSnapshotRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApiVmSnapshotRequestWithDefaults

`func NewApiVmSnapshotRequestWithDefaults() *ApiVmSnapshotRequest`

NewApiVmSnapshotRequestWithDefaults instantiates a new ApiVmSnapshotRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDestinationUrl

`func (o *ApiVmSnapshotRequest) GetDestinationUrl() string`

GetDestinationUrl returns the DestinationUrl field if non-nil, zero value otherwise.

### GetDestinationUrlOk

`func (o *ApiVmSnapshotRequest) GetDestinationUrlOk() (*string, bool)`

GetDestinationUrlOk returns a tuple with the DestinationUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDestinationUrl

`func (o *ApiVmSnapshotRequest) SetDestinationUrl(v string)`

SetDestinationUrl sets DestinationUrl field to given value.

### HasDestinationUrl

`func (o *ApiVmSnapshotRequest) HasDestinationUrl() bool`

HasDestinationUrl returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


