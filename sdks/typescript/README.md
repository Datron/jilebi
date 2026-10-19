# @jilebi/sdk@1.0.0

A TypeScript SDK client for the jilebi.ai API.

## Usage

First, install the SDK from npm.

```bash
npm install @jilebi/sdk --save
```

Next, try it out.


```ts
import {
  Configuration,
  APIKeysApi,
} from '@jilebi/sdk';
import type { DeleteApiV1ApiKeysByIdRequest } from '@jilebi/sdk';

async function example() {
  console.log("🚀 Testing @jilebi/sdk SDK...");
  const config = new Configuration({ 
    // To configure API key authorization: apiKey
    apiKey: "YOUR API KEY",
    // To configure API key authorization: sessionCookie
    apiKey: "YOUR API KEY",
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new APIKeysApi(config);

  const body = {
    // string | Id of the API key.
    id: id_example,
  } satisfies DeleteApiV1ApiKeysByIdRequest;

  try {
    const data = await api.deleteApiV1ApiKeysById(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```


## Documentation

### API Endpoints

All URIs are relative to *https://jilebi.ai*

| Class | Method | HTTP request | Description
| ----- | ------ | ------------ | -------------
*APIKeysApi* | [**deleteApiV1ApiKeysById**](docs/APIKeysApi.md#deleteapiv1apikeysbyid) | **DELETE** /api/v1/api-keys/{id} | Revoke an API key
*APIKeysApi* | [**getApiV1ApiKeys**](docs/APIKeysApi.md#getapiv1apikeys) | **GET** /api/v1/api-keys | List API keys
*APIKeysApi* | [**postApiV1ApiKeys**](docs/APIKeysApi.md#postapiv1apikeys) | **POST** /api/v1/api-keys | Create an API key
*AccountApi* | [**getApiMe**](docs/AccountApi.md#getapime) | **GET** /api/me | Get the current user and session
*BillingApi* | [**getApiV1BillingAdminUsers**](docs/BillingApi.md#getapiv1billingadminusers) | **GET** /api/v1/billing/admin/users | List billable users
*BillingApi* | [**postApiV1BillingAdminGenerateByUsername**](docs/BillingApi.md#postapiv1billingadmingeneratebyusername) | **POST** /api/v1/billing/admin/generate/{username} | Generate an invoice
*BillingApi* | [**postApiV1BillingAdminResetDodoByUsername**](docs/BillingApi.md#postapiv1billingadminresetdodobyusername) | **POST** /api/v1/billing/admin/reset-dodo/{username} | Reset a user\&#39;s Dodo data
*BillingApi* | [**postApiV1BillingPayByBillId**](docs/BillingApi.md#postapiv1billingpaybybillid) | **POST** /api/v1/billing/pay/{billId} | Pay an invoice
*MicroVMsApi* | [**deleteApiV1VmmByMachineName**](docs/MicroVMsApi.md#deleteapiv1vmmbymachinename) | **DELETE** /api/v1/vmm/{machine_name} | Delete a microVM
*MicroVMsApi* | [**getApiV1Vmm**](docs/MicroVMsApi.md#getapiv1vmm) | **GET** /api/v1/vmm | List microVMs
*MicroVMsApi* | [**getApiV1VmmByMachineName**](docs/MicroVMsApi.md#getapiv1vmmbymachinename) | **GET** /api/v1/vmm/{machine_name} | Get a microVM
*MicroVMsApi* | [**postApiV1VmmByMachineName**](docs/MicroVMsApi.md#postapiv1vmmbymachinename) | **POST** /api/v1/vmm/{machine_name} | Create a microVM
*MicroVMsApi* | [**postApiV1VmmByMachineNameClone**](docs/MicroVMsApi.md#postapiv1vmmbymachinenameclone) | **POST** /api/v1/vmm/{machine_name}/clone | Clone a microVM
*MicroVMsApi* | [**putApiV1VmmByMachineName**](docs/MicroVMsApi.md#putapiv1vmmbymachinename) | **PUT** /api/v1/vmm/{machine_name} | Resize a microVM
*MicroVMsApi* | [**putApiV1VmmByMachineNamePause**](docs/MicroVMsApi.md#putapiv1vmmbymachinenamepause) | **PUT** /api/v1/vmm/{machine_name}/pause | Pause a microVM
*MicroVMsApi* | [**putApiV1VmmByMachineNameRestart**](docs/MicroVMsApi.md#putapiv1vmmbymachinenamerestart) | **PUT** /api/v1/vmm/{machine_name}/restart | Restart a microVM
*MicroVMsApi* | [**putApiV1VmmByMachineNameRestore**](docs/MicroVMsApi.md#putapiv1vmmbymachinenamerestore) | **PUT** /api/v1/vmm/{machine_name}/restore | Restore a microVM from a snapshot
*MicroVMsApi* | [**putApiV1VmmByMachineNameShutdown**](docs/MicroVMsApi.md#putapiv1vmmbymachinenameshutdown) | **PUT** /api/v1/vmm/{machine_name}/shutdown | Shut down a microVM
*MicroVMsApi* | [**putApiV1VmmByMachineNameSnapshot**](docs/MicroVMsApi.md#putapiv1vmmbymachinenamesnapshot) | **PUT** /api/v1/vmm/{machine_name}/snapshot | Snapshot a microVM
*MicroVMsApi* | [**putApiV1VmmByMachineNameStart**](docs/MicroVMsApi.md#putapiv1vmmbymachinenamestart) | **PUT** /api/v1/vmm/{machine_name}/start | Start a microVM
*NodesApi* | [**deleteApiV1NodesByName**](docs/NodesApi.md#deleteapiv1nodesbyname) | **DELETE** /api/v1/nodes/{name} | Delete a node
*NodesApi* | [**getApiV1Nodes**](docs/NodesApi.md#getapiv1nodes) | **GET** /api/v1/nodes | List nodes
*NodesApi* | [**getApiV1NodesByName**](docs/NodesApi.md#getapiv1nodesbyname) | **GET** /api/v1/nodes/{name} | Get a node
*NodesApi* | [**postApiV1Nodes**](docs/NodesApi.md#postapiv1nodes) | **POST** /api/v1/nodes | Register a node
*NodesApi* | [**postApiV1NodesByNameHealth**](docs/NodesApi.md#postapiv1nodesbynamehealth) | **POST** /api/v1/nodes/{name}/health | Check node health
*NodesApi* | [**postApiV1NodesByNameRefresh**](docs/NodesApi.md#postapiv1nodesbynamerefresh) | **POST** /api/v1/nodes/{name}/refresh | Refresh node resources
*NodesApi* | [**postApiV1NodesByNameReinitialize**](docs/NodesApi.md#postapiv1nodesbynamereinitialize) | **POST** /api/v1/nodes/{name}/reinitialize | Reinitialize a node
*NodesApi* | [**putApiV1NodesByName**](docs/NodesApi.md#putapiv1nodesbyname) | **PUT** /api/v1/nodes/{name} | Update a node
*RoutesApi* | [**deleteApiV1RoutesByName**](docs/RoutesApi.md#deleteapiv1routesbyname) | **DELETE** /api/v1/routes/{name} | Delete a route
*RoutesApi* | [**getApiV1Routes**](docs/RoutesApi.md#getapiv1routes) | **GET** /api/v1/routes | List routes
*RoutesApi* | [**postApiV1Routes**](docs/RoutesApi.md#postapiv1routes) | **POST** /api/v1/routes | Create a route
*RoutesApi* | [**putApiV1RoutesByName**](docs/RoutesApi.md#putapiv1routesbyname) | **PUT** /api/v1/routes/{name} | Update a route
*SSHKeysApi* | [**deleteApiV1KeysById**](docs/SSHKeysApi.md#deleteapiv1keysbyid) | **DELETE** /api/v1/keys/{id} | Delete an SSH key
*SSHKeysApi* | [**getApiV1Keys**](docs/SSHKeysApi.md#getapiv1keys) | **GET** /api/v1/keys | List SSH keys
*SSHKeysApi* | [**postApiV1Keys**](docs/SSHKeysApi.md#postapiv1keys) | **POST** /api/v1/keys | Add an SSH key
*SSHKeysApi* | [**putApiV1KeysById**](docs/SSHKeysApi.md#putapiv1keysbyid) | **PUT** /api/v1/keys/{id} | Rename an SSH key
*SoftwareCatalogApi* | [**deleteApiV1SoftwareById**](docs/SoftwareCatalogApi.md#deleteapiv1softwarebyid) | **DELETE** /api/v1/software/{id} | Delete software
*SoftwareCatalogApi* | [**getApiV1Software**](docs/SoftwareCatalogApi.md#getapiv1software) | **GET** /api/v1/software | List software
*SoftwareCatalogApi* | [**getApiV1SoftwareById**](docs/SoftwareCatalogApi.md#getapiv1softwarebyid) | **GET** /api/v1/software/{id} | Get software
*SoftwareCatalogApi* | [**postApiV1Software**](docs/SoftwareCatalogApi.md#postapiv1software) | **POST** /api/v1/software | Add software
*SoftwareCatalogApi* | [**putApiV1SoftwareById**](docs/SoftwareCatalogApi.md#putapiv1softwarebyid) | **PUT** /api/v1/software/{id} | Update software


### Models

- [ApiBillingGenerateResponse](docs/ApiBillingGenerateResponse.md)
- [ApiBillingGenerateResponseBillsInner](docs/ApiBillingGenerateResponseBillsInner.md)
- [ApiBillingResetDodoResponse](docs/ApiBillingResetDodoResponse.md)
- [ApiBillingUser](docs/ApiBillingUser.md)
- [ApiBillingUsersResponse](docs/ApiBillingUsersResponse.md)
- [ApiError](docs/ApiError.md)
- [ApiKey](docs/ApiKey.md)
- [ApiKeyCreateRequest](docs/ApiKeyCreateRequest.md)
- [ApiKeyCreateRequestExpiresIn](docs/ApiKeyCreateRequestExpiresIn.md)
- [ApiKeyCreateResponse](docs/ApiKeyCreateResponse.md)
- [ApiMeResponse](docs/ApiMeResponse.md)
- [ApiNode](docs/ApiNode.md)
- [ApiNodeCreateRequest](docs/ApiNodeCreateRequest.md)
- [ApiNodeHealthResponse](docs/ApiNodeHealthResponse.md)
- [ApiNodeRefreshResponse](docs/ApiNodeRefreshResponse.md)
- [ApiNodeReinitializeResponse](docs/ApiNodeReinitializeResponse.md)
- [ApiNodeStatus](docs/ApiNodeStatus.md)
- [ApiNodeUpdateRequest](docs/ApiNodeUpdateRequest.md)
- [ApiRoute](docs/ApiRoute.md)
- [ApiRouteCreateRequest](docs/ApiRouteCreateRequest.md)
- [ApiRouteCreateResponse](docs/ApiRouteCreateResponse.md)
- [ApiRouteListResponse](docs/ApiRouteListResponse.md)
- [ApiRouteUpdateRequest](docs/ApiRouteUpdateRequest.md)
- [ApiSoftware](docs/ApiSoftware.md)
- [ApiSoftwareCreateRequest](docs/ApiSoftwareCreateRequest.md)
- [ApiSoftwareUpdateRequest](docs/ApiSoftwareUpdateRequest.md)
- [ApiSshKey](docs/ApiSshKey.md)
- [ApiSshKeyCreateRequest](docs/ApiSshKeyCreateRequest.md)
- [ApiSshKeyCreateResponse](docs/ApiSshKeyCreateResponse.md)
- [ApiSshKeyUpdateRequest](docs/ApiSshKeyUpdateRequest.md)
- [ApiSuccess](docs/ApiSuccess.md)
- [ApiVm](docs/ApiVm.md)
- [ApiVmCloneRequest](docs/ApiVmCloneRequest.md)
- [ApiVmCloneResponse](docs/ApiVmCloneResponse.md)
- [ApiVmCreateRequest](docs/ApiVmCreateRequest.md)
- [ApiVmCreateResponse](docs/ApiVmCreateResponse.md)
- [ApiVmGetResponse](docs/ApiVmGetResponse.md)
- [ApiVmRestoreRequest](docs/ApiVmRestoreRequest.md)
- [ApiVmRestoreResponse](docs/ApiVmRestoreResponse.md)
- [ApiVmSnapshotRequest](docs/ApiVmSnapshotRequest.md)
- [ApiVmSnapshotResponse](docs/ApiVmSnapshotResponse.md)
- [ApiVmState](docs/ApiVmState.md)
- [ApiVmUpdateRequest](docs/ApiVmUpdateRequest.md)
- [DeviceNode](docs/DeviceNode.md)
- [ProvisionedVm](docs/ProvisionedVm.md)
- [SharedUser](docs/SharedUser.md)
- [VmInfo](docs/VmInfo.md)

### Authorization


Authentication schemes defined for the API:
<a id="apiKey"></a>
#### apiKey


- **Type**: API key
- **API key parameter name**: `X-API-Key`
- **Location**: HTTP header
<a id="bearerAuth"></a>
#### bearerAuth


- **Type**: HTTP Bearer Token authentication (API key)
<a id="sessionCookie"></a>
#### sessionCookie


- **Type**: API key
- **API key parameter name**: `better-auth.session_token`
- **Location**: 

## About

This TypeScript SDK client supports the [Fetch API](https://fetch.spec.whatwg.org/)
and is automatically generated by the
[OpenAPI Generator](https://openapi-generator.tech) project:

- API version: `1.0.0`
- Package version: `1.0.0`
- Generator version: `7.26.0`
- Build package: `org.openapitools.codegen.languages.TypeScriptFetchClientCodegen`

The generated npm module supports the following:

- Environments
  * Node.js
  * Webpack
  * Browserify
- Language levels
  * ES5 - you must have a Promises/A+ library installed
  * ES6
- Module systems
  * CommonJS
  * ES6 module system


## Development

### Building

To build the TypeScript source code, you need to have Node.js and npm installed.
After cloning the repository, navigate to the project directory and run:

```bash
npm install
npm run build
```

### Publishing

Once you've built the package, you can publish it to npm:

```bash
npm publish
```

## License

[AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.html)
