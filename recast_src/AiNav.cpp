
#include "AiNav.h"


void TestReturnArray(DtCrowdAgentsResult* result)
{
	float value = 1000.0f;
	for (int i = 0; i < 10; ++i)
	{
		DtCrowdAgent ca;
		ca.index = i;
		ca.active = 1;
		ca.desiredSpeed = 20.0f;
		ca.partial = 5;
		ca.position.x = value + 100.0f;
		ca.position.y = value + 200.0f;
		ca.position.z = value + 300.0f;

		result->agents[i] = ca;
		value += 100.0f;
	}
	result->agentCount = 10;
}

int GetVersion()
{
	return 1;
}

void test_method() {

}

int test_return_vector(float3* value1, float3 value2)
{
	if (value1->x != 123 || value1->y != 234 || value1->z != 345) {
		return 0;
	}
	if (value2.x != 321 || value2.y != 432 || value2.z != 543) {
		return 1;
	}

	value1->x = value2.x;
	value1->y = value2.y;
	value1->z = value2.z;

	return 2;
}

NavigationBuilder* CreateBuilder() {
	return new NavigationBuilder();
}

void DestroyBuilder(NavigationBuilder* nav)
{
	delete nav;
}

void SetSettings(NavigationBuilder* nav, DtBuildSettings* buildSettings)
{
	nav->SetSettings(*buildSettings);
}

DtGeneratedData* BuildNavmesh(NavigationBuilder* nav,
	float3* vertices, int numVertices,
	int* indices, int numIndices, uint8_t* areas)
{
	return nav->BuildNavmesh(vertices, numVertices, indices, numIndices, areas);
}

// Navmesh
void* CreateNavmesh(float cellTileSize, int tileBits, int polyBits)
{
	NavigationMesh* navmesh = new NavigationMesh();
	if (!navmesh->Init(cellTileSize, tileBits, polyBits))
	{
		delete navmesh;
		navmesh = nullptr;
	}
	return navmesh;
}

void DestroyNavmesh(NavigationMesh* navmesh)
{
	delete navmesh;
}

int AddTile(NavigationMesh* navmesh, uint8_t* data, int dataLength)
{
	return navmesh->LoadTile(data, dataLength);
}

int RemoveTile(NavigationMesh* navmesh, int2* tileCoordinate)
{
	int2 coord = *tileCoordinate;
	return navmesh->RemoveTile(coord);
}

// Query

void* QueryCreate(NavigationMesh* navmesh, int maxNodes)
{
	AiQuery* aiQuery = new AiQuery();
	if (!aiQuery->Init(navmesh, maxNodes))
	{
		delete aiQuery;
		aiQuery = nullptr;
	}
	return aiQuery;
}

void QueryDestroy(AiQuery* aiQuery)
{
	delete aiQuery;
}

int QueryIsValid(AiQuery* aiQuery)
{
	return aiQuery->IsValid();
}

void QueryInvalidate(AiQuery* aiQuery)
{
	aiQuery->Invalidate();
}

void QueryFindStraightPath(AiQuery* aiQuery, NavMeshPathfindQuery* query, NavMeshPathfindResult* result)
{
	aiQuery->FindStraightPath(*query, result);
}

int QueryHasPath(AiQuery* aiQuery, NavMeshPathfindQuery* query)
{
	return aiQuery->HasPath(*query);
}

void QueryRaycast(AiQuery* aiQuery, NavMeshRaycastQuery* query, NavMeshRaycastResult* result)
{
	aiQuery->Raycast(*query, result);
}

int QuerySamplePosition(AiQuery* aiQuery, float3* point, float3* extent, float3* result)
{
	return aiQuery->SamplePosition(*point, *extent, result);
}

int QueryGetRandomPosition(AiQuery* aiQuery, float3* result)
{
	return aiQuery->GetRandomPosition(result);
}

int QueryGetLocation(AiQuery* aiQuery, float3* point, float3* extent, float3* result)
{
	return aiQuery->GetLocation(*point, *extent, result);
}

// Crowd

int CrowdGetVersion(AiCrowd* crowd)
{
	return crowd->GetVersion();
}

void* CrowdCreate(NavigationMesh* navmesh, int maxAgents, float maxAgentRadius)
{
	AiCrowd* crowd = new AiCrowd();
	if (!crowd->Init(navmesh, maxAgents, maxAgentRadius))
	{
		delete crowd;
		crowd = nullptr;
	}
	return crowd;
}

void CrowdDestroy(AiCrowd* crowd)
{
	delete crowd;
}

void CrowdGetAgents(AiCrowd* crowd, DtCrowdAgentsResult* result)
{
	crowd->GetActiveAgents(result);
}

int CrowdGetAgent(AiCrowd* crowd, int idx, DtCrowdAgent* result)
{
	return crowd->GetAgent(idx, result);
}

int CrowdAddAgent(AiCrowd* crowd, float3* position, DtAgentParams* params)
{
	return crowd->AddAgent(*position, params);
}

void CrowdRemoveAgent(AiCrowd* crowd, int idx)
{
	crowd->RemoveAgent(idx);
}

void CrowdSetAgentParams(AiCrowd* crowd, int idx, DtAgentParams* agentParams)
{
	crowd->SetAgentParams(idx, agentParams);
}

void CrowdGetAgentParams(AiCrowd* crowd, int idx, DtAgentParams* agentParams)
{
	crowd->GetAgentParams(idx, agentParams);
}

int CrowdGetNearestPoly(AiCrowd* crowd, int idx, float3* position, uint64_t* nearestPoly, float3* nearestPoint)
{
	return crowd->GetNearestPoly(idx, *position, nearestPoly, nearestPoint);
}

int CrowdRequestMoveAgentByPoly(AiCrowd* crowd, int idx, uint64_t* polyRef, float3* position)
{
	return crowd->RequestMoveByPoly(idx, *polyRef, *position);
}

int CrowdRequestMoveAgent(AiCrowd* crowd, int idx, float3* position)
{
	return crowd->RequestMove(idx, *position);
}

int CrowdGetAgentCount(AiCrowd* crowd)
{
	return crowd->GetAgentCount();
}

void CrowdUpdate(AiCrowd* crowd, const float dt)
{
	crowd->Update(dt);
}
