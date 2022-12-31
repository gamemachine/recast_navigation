#pragma once
#include <cstring>
#include <DetourCrowd.h>
#include "NavigationMesh.hpp"

class AiCrowd {
private:
	int activeAgentCount = 0;
	dtNavMesh* m_navMesh = nullptr;
	dtNavMeshQuery* m_navQuery = nullptr;
	dtCrowd* crowd = nullptr;
	dtCrowdAgentParams CreateParams(DtAgentParams* agentParams);
public:
	AiCrowd();
	~AiCrowd();
	int GetVersion();
	int Init(NavigationMesh * navmesh, int maxAgents, float maxRadius);
	int AddAgent(float3 position, DtAgentParams * agentParams);
	void RemoveAgent(int idx);
	void SetAgentParams(int idx, DtAgentParams* agentParams);
	void GetAgentParams(int idx, DtAgentParams* agentParams);
	int RequestMove(int idx, float3 position);
	int RequestMoveByPoly(int idx, uint64_t polyRef, float3 position);
	int GetNearestPoly(int idx, float3 position, uint64_t* polyRef, float3* nearestPoint);
	int GetAgentCount();
	int GetAgent(int idx, DtCrowdAgent* result);
	void GetActiveAgents(DtCrowdAgentsResult* result);
	void Update(const float dt);
};