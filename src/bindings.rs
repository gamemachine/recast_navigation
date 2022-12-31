use crate::building::*;
use crate::common::*;
use crate::crowd::DtAgentParams;
use crate::crowd::DtCrowdAgent;
use crate::crowd::DtCrowdAgentsResult;
use crate::queries::DtPathFindQuery;
use crate::queries::DtPathFindResult;
use crate::queries::DtRaycastQuery;
use crate::queries::DtRaycastResult;

/// Opaque types - https://anssi-fr.github.io/rust-guide/07_ffi.html
#[repr(C)]
pub struct RawBuilderPtr {_private: [u8; 0]}

#[repr(C)]
pub struct RawNavmeshPtr {_private: [u8; 0]}

#[repr(C)]
pub struct RawNavqueryPtr {_private: [u8; 0]}

#[repr(C)]
pub struct RawCrowdPtr {_private: [u8; 0]}

#[link(name = "AiNav")]
extern "C" {
    pub fn CreateBuilder() -> *mut RawBuilderPtr;
    pub fn DestroyBuilder(builder: *mut RawBuilderPtr);

    pub fn SetSettings(builder: *mut RawBuilderPtr, buildSettings: *mut DtBuildSettings);
    pub fn BuildNavmesh(builder: *mut RawBuilderPtr, vertices: *mut DtVector, numVertices: i32, indices: *mut i32, numIndices: i32, areas: *mut u8) -> *const DtGeneratedData;
    
    pub fn CreateNavmesh(cellTileSize: f32, tileBits: i32, polyBits: i32) -> *mut RawNavmeshPtr;
    pub fn DestroyNavmesh(navmesh: *mut RawNavmeshPtr);

    pub fn AddTile(navmesh: *mut RawNavmeshPtr, data: *mut u8, dataLength: i32) -> i32;
    pub fn RemoveTile(navmesh: *mut RawNavmeshPtr, tileCoordinate: *const DtInt2) -> i32;

    pub fn QueryCreate(navmesh: *mut RawNavmeshPtr, maxNodes: i32) -> *mut RawNavqueryPtr;
    pub fn QueryDestroy(query: *mut RawNavqueryPtr);
    pub fn QueryInvalidate(query: *mut RawNavqueryPtr);

    pub fn QueryFindStraightPath(query: *mut RawNavqueryPtr, query: *const DtPathFindQuery, result: *mut DtPathFindResult);
    pub fn QuerySamplePosition(query: *mut RawNavqueryPtr, point: *const DtVector, extent: *const DtVector, result: *mut DtVector) -> i32;
    pub fn QueryGetLocation(query: *mut RawNavqueryPtr, point: *const DtVector, extent: *const DtVector, result: *mut DtVector) -> i32;
    pub fn QueryRaycast(query: *mut RawNavqueryPtr, query: *const DtRaycastQuery, result: *mut DtRaycastResult);
    pub fn QueryHasPath(query: *mut RawNavqueryPtr, query: *const DtPathFindQuery) -> i32;

    pub fn CrowdCreate(navmesh: *mut RawNavmeshPtr, max_agents: i32, max_agent_radius: f32) -> *mut RawCrowdPtr;
    pub fn CrowdDestroy(crowd: *mut RawCrowdPtr);
    /// returns the index of the agent.  Or -1 if agent could not be added
    pub fn CrowdAddAgent(crowd: *mut RawCrowdPtr, position: *const DtVector, agent_params: *const DtAgentParams) -> i32;
    pub fn CrowdRemoveAgent(crowd: *mut RawCrowdPtr, idx: i32);
    pub fn CrowdGetAgentCount(crowd: *mut RawCrowdPtr) -> i32;
    pub fn CrowdSetAgentParams(crowd: *mut RawCrowdPtr, idx: i32,agent_params: *const DtAgentParams);
    pub fn CrowdGetAgentParams(crowd: *mut RawCrowdPtr, idx: i32,agent_params: *mut DtAgentParams);
    // returns 1 on success
    pub fn CrowdRequestMoveAgent(crowd: *mut RawCrowdPtr, idx: i32,position: *const DtVector) -> i32;
    pub fn CrowdUpdate(crowd: *mut RawCrowdPtr, delta_time: f32);

    /// returns 1 on success
    pub fn CrowdGetAgent(crowd: *mut RawCrowdPtr, idx: i32, agent: *mut DtCrowdAgent) -> i32;

    // returns active agents
    pub fn CrowdGetAgents(crowd: *mut RawCrowdPtr, agents: *mut DtCrowdAgentsResult);
}