#include "AiCrowd.hpp"

AiCrowd::AiCrowd()
{
	crowd = dtAllocCrowd();
}

AiCrowd::~AiCrowd()
{
	dtFreeCrowd(crowd);
}

int AiCrowd::GetVersion()
{
	return 1;
}

int AiCrowd::Init(NavigationMesh * navmesh, int maxAgents, float maxRadius)
{
	m_navMesh = navmesh->GetDtNavMesh();
	m_navQuery = navmesh->GetNavmeshQuery();

	crowd->init(maxAgents, maxRadius, m_navMesh);

	dtObstacleAvoidanceParams params;
	memcpy(&params, crowd->getObstacleAvoidanceParams(0), sizeof(dtObstacleAvoidanceParams));

	// Low (11)
	params.velBias = 0.5f;
	params.adaptiveDivs = 5;
	params.adaptiveRings = 2;
	params.adaptiveDepth = 1;
	crowd->setObstacleAvoidanceParams(0, &params);

	// Medium (22)
	params.velBias = 0.5f;
	params.adaptiveDivs = 5;
	params.adaptiveRings = 2;
	params.adaptiveDepth = 2;
	crowd->setObstacleAvoidanceParams(1, &params);

	// Good (45)
	params.velBias = 0.5f;
	params.adaptiveDivs = 7;
	params.adaptiveRings = 2;
	params.adaptiveDepth = 3;
	crowd->setObstacleAvoidanceParams(2, &params);

	// High (66)
	params.velBias = 0.5f;
	params.adaptiveDivs = 7;
	params.adaptiveRings = 3;
	params.adaptiveDepth = 3;

	crowd->setObstacleAvoidanceParams(3, &params);


	return 1;
}

int AiCrowd::AddAgent(float3 position, DtAgentParams* agentParams)
{
	const dtCrowdAgentParams ap = CreateParams(agentParams);
	float addPos[3];
	addPos[0] = position.x;
	addPos[1] = position.y;
	addPos[2] = position.z;

	int id = crowd->addAgent(addPos, &ap);
	if (id != -1) {
		activeAgentCount++;
	}
	return id;
}

void AiCrowd::RemoveAgent(int idx)
{
	const dtCrowdAgent* ag = crowd->getAgent(idx);
	if (ag->active)
	{
		crowd->removeAgent(idx);
		activeAgentCount--;
	}
}

void AiCrowd::SetAgentParams(int idx, DtAgentParams* agentParams)
{
	const dtCrowdAgentParams ap = CreateParams(agentParams);
	crowd->updateAgentParameters(idx, &ap);
}

void AiCrowd::GetAgentParams(int idx, DtAgentParams* agentParams)
{
	const dtCrowdAgent* ag = crowd->getAgent(idx);
	
	agentParams->radius = ag->params.radius;
	agentParams->height = ag->params.height;
	agentParams->maxAcceleration = ag->params.maxAcceleration;
	agentParams->maxSpeed = ag->params.maxSpeed;
	agentParams->collisionQueryRange = ag->params.collisionQueryRange;
	agentParams->pathOptimizationRange = ag->params.pathOptimizationRange;
	agentParams->obstacleAvoidanceType = ag->params.obstacleAvoidanceType;
	agentParams->separationWeight = ag->params.separationWeight;

	agentParams->anticipateTurns = ag->params.updateFlags & DT_CROWD_ANTICIPATE_TURNS ? 1 : 0;
	agentParams->optimizeVis = ag->params.updateFlags & DT_CROWD_OPTIMIZE_VIS ? 1 : 0;
	agentParams->optimizeTopo = ag->params.updateFlags & DT_CROWD_OPTIMIZE_TOPO ? 1 : 0;
	agentParams->obstacleAvoidance = ag->params.updateFlags & DT_CROWD_OBSTACLE_AVOIDANCE ? 1 : 0;
	agentParams->crowdSeparation = ag->params.updateFlags & DT_CROWD_SEPARATION ? 1 : 0;
}

int AiCrowd::GetNearestPoly(int idx, float3 position, uint64_t* nearestPoly, float3* nearestPoint)
{
	dtPolyRef startPoly;
	float m_targetPos[3];
	const dtQueryFilter* filter = crowd->getFilter(0);
	const float* halfExtents = crowd->getQueryExtents();

	dtStatus status;
	status = m_navQuery->findNearestPoly(&position.x, halfExtents, filter, &startPoly, m_targetPos);
	if (dtStatusFailed(status) || startPoly == 0) {
		return 0;
	}

	nearestPoly = &startPoly;
	nearestPoint->x = m_targetPos[0];
	nearestPoint->y = m_targetPos[1];
	nearestPoint->z = m_targetPos[2];

	return 1;
}

int AiCrowd::RequestMoveByPoly(int idx, uint64_t polyRef, float3 position)
{
	float m_targetPos[3];
	m_targetPos[0] = position.x;
	m_targetPos[1] = position.y;
	m_targetPos[2] = position.z;

	const dtCrowdAgent* ag = crowd->getAgent(idx);
	if (ag && ag->active)
	{
		bool moveStatus = crowd->requestMoveTarget(idx, polyRef, m_targetPos);
		return moveStatus ? 1 : 0;
	}

	return 0;

}

int AiCrowd::RequestMove(int idx, float3 position)
{
	dtPolyRef startPoly;
	float m_targetPos[3];
	const dtQueryFilter* filter = crowd->getFilter(0);
	const float* halfExtents = crowd->getQueryExtents();

	dtStatus status;
	status = m_navQuery->findNearestPoly(&position.x, halfExtents, filter, &startPoly, m_targetPos);
	if (dtStatusFailed(status) || startPoly == 0) {
		return 0;
	}
	const dtCrowdAgent* ag = crowd->getAgent(idx);
	if (ag && ag->active)
	{
		bool moveStatus = crowd->requestMoveTarget(idx, startPoly, m_targetPos);
		return moveStatus ? 1 : 0;
	}

	return 0;
		
}



void AiCrowd::GetActiveAgents(DtCrowdAgentsResult * result)
{
	int index = 0;
	int agentCount = crowd->getAgentCount();
	for (int i = 0; i < agentCount; ++i)
	{
		const dtCrowdAgent* ag = crowd->getAgent(i);
		if (!ag->active) continue;
		
		DtCrowdAgent ca = result->agents[index];
		ca.index = i;
		ca.active = ag->active ? 1 : 0;
		ca.desiredSpeed = ag->desiredSpeed;
		ca.partial = ag->partial ? 1 : 0;
		ca.position.x = ag->npos[0];
		ca.position.y = ag->npos[1];
		ca.position.z = ag->npos[2];
		
		result->agents[index] = ca;

		index++;
	}
	result->agentCount = index;
}

int AiCrowd::GetAgentCount()
{
	return activeAgentCount;
}

int AiCrowd::GetAgent(int idx, DtCrowdAgent* result)
{
	const dtCrowdAgent* ag = crowd->getAgent(idx);
	if (ag == 0) {
		return 0;
	}
	result->index = idx;
	result->state = ag->state;
	result->active = ag->active ? 1 : 0;
	result->desiredSpeed = ag->desiredSpeed;
	result->partial = ag->partial ? 1 : 0;

	result->position.x = ag->npos[0];
	result->position.y = ag->npos[1];
	result->position.z = ag->npos[2];

	result->velocity.x = ag->vel[0];
	result->velocity.y = ag->vel[1];
	result->velocity.z = ag->vel[2];

	return 1;
}

void AiCrowd::Update(const float dt)
{
	//dtCrowdAgentDebugInfo debug;
	crowd->update(dt, nullptr);
}

dtCrowdAgentParams AiCrowd::CreateParams(DtAgentParams* agentParams)
{
	dtCrowdAgentParams ap;
	memset(&ap, 0, sizeof(ap));

	ap.radius = agentParams->radius;
	ap.height = agentParams->height;
	ap.maxAcceleration = agentParams->maxAcceleration;
	ap.maxSpeed = agentParams->maxSpeed;
	ap.collisionQueryRange = agentParams->collisionQueryRange;
	ap.pathOptimizationRange = agentParams->pathOptimizationRange;
	ap.obstacleAvoidanceType = agentParams->obstacleAvoidanceType;
	ap.separationWeight = agentParams->separationWeight;

	ap.updateFlags = 0;
	if (agentParams->anticipateTurns)
		ap.updateFlags |= DT_CROWD_ANTICIPATE_TURNS;
	if (agentParams->optimizeVis)
		ap.updateFlags |= DT_CROWD_OPTIMIZE_VIS;
	if (agentParams->optimizeTopo)
		ap.updateFlags |= DT_CROWD_OPTIMIZE_TOPO;
	if (agentParams->obstacleAvoidance)
		ap.updateFlags |= DT_CROWD_OBSTACLE_AVOIDANCE;
	if (agentParams->crowdSeparation)
		ap.updateFlags |= DT_CROWD_SEPARATION;

	return ap;
}


