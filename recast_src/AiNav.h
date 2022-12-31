#pragma once
#include "DetourNavMeshBuilder.h"
#include "NavigationBuilder.hpp"
#include "NavigationMesh.hpp"
#include "AiCrowd.hpp"
#include "AiQuery.hpp"

#ifdef _WIN32
#define AINAV_API __declspec(dllexport)
#elif defined(__clang__) || defined(__GNUC__)
#define AINAV_API __attribute__((visibility("default")))
#else
#define AINAV_API
#endif // _WIN32

extern "C" AINAV_API int GetVersion();
extern "C" AINAV_API void test_method();
extern "C" AINAV_API int test_return_vector(float3* value, float3 value2);
extern "C" AINAV_API void TestReturnArray(DtCrowdAgentsResult * result);

extern "C" AINAV_API NavigationBuilder * CreateBuilder();
extern "C" AINAV_API void DestroyBuilder(NavigationBuilder * nav);
extern "C" AINAV_API void SetSettings(NavigationBuilder * nav, DtBuildSettings * buildSettings);
extern "C" AINAV_API DtGeneratedData * BuildNavmesh(NavigationBuilder * nav, float3 * vertices, int numVertices, int* indices, int numIndices, uint8_t* areas);
extern "C" AINAV_API void* CreateNavmesh(float cellTileSize, int tileBits, int polyBits);
extern "C" AINAV_API void DestroyNavmesh(NavigationMesh * navmesh);
extern "C" AINAV_API int AddTile(NavigationMesh * navmesh, uint8_t * data, int dataLength);
extern "C" AINAV_API int RemoveTile(NavigationMesh * navmesh, int2* tileCoordinate);


extern "C" AINAV_API void* QueryCreate(NavigationMesh * navmesh, int maxNodes);
extern "C" AINAV_API void QueryDestroy(AiQuery * aiQuery);
extern "C" AINAV_API void QueryInvalidate(AiQuery * aiQuery);
extern "C" AINAV_API int QueryIsValid(AiQuery * aiQuery);
extern "C" AINAV_API void QueryFindStraightPath(AiQuery * aiQuery, NavMeshPathfindQuery* query, NavMeshPathfindResult * result);
extern "C" AINAV_API int QueryHasPath(AiQuery * aiQuery, NavMeshPathfindQuery* query);
extern "C" AINAV_API void QueryRaycast(AiQuery * aiQuery, NavMeshRaycastQuery* query, NavMeshRaycastResult * result);
extern "C" AINAV_API int QuerySamplePosition(AiQuery * aiQuery, float3* point, float3* extent, float3 * result);
extern "C" AINAV_API int QueryGetRandomPosition(AiQuery * aiQuery, float3 * result);
extern "C" AINAV_API int QueryGetLocation(AiQuery * aiQuery, float3* point, float3* extent, float3 * result);

extern "C" AINAV_API int CrowdGetVersion(AiCrowd * crowd);
extern "C" AINAV_API void* CrowdCreate(NavigationMesh * navmesh, int maxAgents, float maxAgentRadius);
extern "C" AINAV_API void CrowdDestroy(AiCrowd * crowd);
extern "C" AINAV_API int CrowdAddAgent(AiCrowd * crowd, float3* position, DtAgentParams * params);
extern "C" AINAV_API void CrowdRemoveAgent(AiCrowd * crowd, int idx);
extern "C" AINAV_API int CrowdGetAgentCount(AiCrowd * crowd);
extern "C" AINAV_API void CrowdSetAgentParams(AiCrowd * crowd, int idx, DtAgentParams * agentParams);
extern "C" AINAV_API void CrowdGetAgentParams(AiCrowd * crowd, int idx, DtAgentParams * agentParams);
extern "C" AINAV_API int CrowdRequestMoveAgent(AiCrowd * crowd, int idx, float3* position);
extern "C" AINAV_API int CrowdRequestMoveAgentByPoly(AiCrowd * crowd, int idx, uint64_t* polyRef, float3 * position);
extern "C" AINAV_API int CrowdGetNearestPoly(AiCrowd * crowd, int idx, float3* position, uint64_t * nearestPoly, float3 * nearestPoint);
extern "C" AINAV_API void CrowdUpdate(AiCrowd * crowd, const float dt);
extern "C" AINAV_API int CrowdGetAgent(AiCrowd * crowd, int idx, DtCrowdAgent * result);
extern "C" AINAV_API void CrowdGetAgents(AiCrowd * crowd, DtCrowdAgentsResult * result);
