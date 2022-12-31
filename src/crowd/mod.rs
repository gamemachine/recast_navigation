use crate::common::DtVector;


pub mod crowd;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtAgentParams {
    ///< Agent radius. [Limit: >= 0]
	pub radius: f32,
    ///< Agent height. [Limit: > 0]			
	pub height: f32,
    ///< Maximum allowed acceleration. [Limit: >= 0]						
	pub max_acceleration: f32,
    ///< Maximum allowed speed. [Limit: >= 0]			
	pub max_speed: f32,						
	pub collision_query_range: f32,
    ///< The path visibility optimization range. [Limit: > 0]
	pub path_optimization_range: f32,		
	pub separation_weight: f32,
	pub anticipate_turns: i32,
	pub optimize_vis: i32,
	pub optimize_topo: i32,
	pub obstacle_avoidance: i32,
	pub crowd_separation: i32,
    /// Index into one of 4 presets set in AiCrowd.cpp.  Indexes 0-4.
	pub obstacle_avoidance_type: i32,
	pub query_filter_type: i32
}

impl DtAgentParams {
    pub fn default() -> Self {
        DtAgentParams {
            radius: 0.5,
            height: 2.0,
            max_acceleration: 20.0,
            max_speed: 5.0,
            collision_query_range: 6.0,
            path_optimization_range: 15.0,
            separation_weight: 1.0,
            anticipate_turns: 1,
            optimize_vis: 1,
            optimize_topo: 1,
            obstacle_avoidance: 1,
            crowd_separation: 1,
            obstacle_avoidance_type: 1,
            query_filter_type: 0,
        }
    }
}

pub enum CrowdAgentState
{
    //< The agent is not in a valid state.
	DtCrowdagentStateInvalid,
    //< The agent is traversing a normal navigation mesh polygon.		
	DtCrowdagentStateWalking,
    //< The agent is traversing an off-mesh connection.		
	DtCrowdagentStateOffmesh		
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct DtCrowdAgent
{
    pub index: i32,
	pub state: u8,
	pub active: i32,
	pub partial: i32,
	pub desired_speed: f32,
	pub position: DtVector,
	pub velocity: DtVector
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DtCrowdAgentsResult
{
	pub agents: *mut DtCrowdAgent,
	pub agent_count: i32
}