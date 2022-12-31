use crate::{
    bindings::{
        CrowdAddAgent, CrowdCreate, CrowdDestroy, CrowdGetAgent, CrowdGetAgentCount,
        CrowdGetAgentParams, CrowdGetAgents, CrowdRemoveAgent, CrowdRequestMoveAgent,
        CrowdSetAgentParams, CrowdUpdate, RawCrowdPtr, RawNavmeshPtr,
    },
    common::DtVector,
};

use super::{DtAgentParams, DtCrowdAgent, DtCrowdAgentsResult};

struct CrowdPtr(*mut RawCrowdPtr);
unsafe impl Send for CrowdPtr {}

pub struct Crowd {
    crowd_ptr: CrowdPtr,
    max_agents: i32,
}

impl Crowd {
    pub fn new(
        navmesh_ptr: *mut RawNavmeshPtr,
        max_agents: i32,
        max_agent_radius: f32,
    ) -> Option<Self> {
        
        let ptr = unsafe { CrowdCreate(navmesh_ptr, max_agents, max_agent_radius) };
        if ptr.is_null() {
            return None;
        }

        let crowd_ptr = CrowdPtr(ptr);
        let crowd = Crowd {
            crowd_ptr,
            max_agents,
        };
        Some(crowd)
    }

    pub fn update(&mut self, delta_time: f32) {
        unsafe { CrowdUpdate(self.crowd_ptr.0, delta_time) }
    }

    pub fn move_agent(&mut self, idx: i32, position: DtVector) -> bool {
        unsafe { CrowdRequestMoveAgent(self.crowd_ptr.0, idx, &position as *const DtVector) == 1 }
    }

    pub fn get_agent(&mut self, idx: i32) -> Option<DtCrowdAgent> {
        unsafe {
            let mut agent = DtCrowdAgent::default();
            let res = CrowdGetAgent(self.crowd_ptr.0, idx, &mut agent as *mut DtCrowdAgent);
            if res == 1 {
                Some(agent)
            } else {
                None
            }
        }
    }

    pub fn get_agents(&mut self) -> Option<Vec<DtCrowdAgent>> {
        unsafe {
            let mut agents = vec![DtCrowdAgent::default(); self.max_agents as usize];
            let mut result = DtCrowdAgentsResult {
                agents: agents.as_mut_ptr() as *mut DtCrowdAgent,
                agent_count: 0,
            };

            CrowdGetAgents(self.crowd_ptr.0, &mut result as *mut DtCrowdAgentsResult);
            if result.agent_count > 0 {
                agents.truncate(result.agent_count as usize);
                Some(agents)
            } else {
                None
            }
        }
    }

    pub fn add_agent(&mut self, position: DtVector, params: DtAgentParams) -> Option<i32> {
        let res = unsafe {
            CrowdAddAgent(
                self.crowd_ptr.0,
                &position as *const DtVector,
                &params as *const DtAgentParams,
            )
        };
        if res >= 0 {
            Some(res)
        } else {
            None
        }
    }

    pub fn remove_agent(&mut self, idx: i32) {
        unsafe { CrowdRemoveAgent(self.crowd_ptr.0, idx) }
    }

    pub fn agent_count(&mut self) -> i32 {
        unsafe { CrowdGetAgentCount(self.crowd_ptr.0) }
    }

    pub fn set_agent_params(&mut self, idx: i32, params: DtAgentParams) {
        unsafe { CrowdSetAgentParams(self.crowd_ptr.0, idx, &params as *const DtAgentParams) }
    }

    pub fn get_agent_params(&mut self, idx: i32) -> DtAgentParams {
        unsafe {
            let mut params = DtAgentParams::default();
            CrowdGetAgentParams(self.crowd_ptr.0, idx, &mut params as *mut DtAgentParams);
            params
        }
    }
}

impl Drop for Crowd {
    fn drop(&mut self) {
        unsafe { CrowdDestroy(self.crowd_ptr.0) };
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        building::{navmesh_builder::NavmeshBuilder, NavBuildSettings},
        common::{
            navmesh::{Navmesh, NavmeshSettings},
            DtVector,
        },
        crowd::DtAgentParams,
    };

    use super::Crowd;

    #[test]
    fn crowd_flow() {
        let build_settings = NavBuildSettings::default();
        let navmesh_settings = NavmeshSettings::default(build_settings, 2048.0, 10);
        let mut navmesh = Navmesh::new(navmesh_settings).unwrap();

        let result = NavmeshBuilder::build_test_tile(100.0);
        let tile = result.tile.unwrap();
        navmesh.add_or_replace_tile(tile);

        let mut crowd = Crowd::new(navmesh.raw_ptr(), 100, 0.5).unwrap();

        let pos = DtVector::new(1.0, 1.0, 1.0);
        let params = DtAgentParams::default();

        let idx0 = crowd.add_agent(pos, params).unwrap();
        let idx1 = crowd.add_agent(pos, params).unwrap();

        let count = crowd.agent_count();
        assert_eq!(count, 2);

        let move_pos = DtVector::new(10.0, 1.0, 10.0);
        assert!(crowd.move_agent(idx0, move_pos));

        for _ in 0..10 {
            crowd.update(0.2);
        }

        let agent = crowd.get_agent(idx0).unwrap();
        assert_eq!(idx0, agent.index);

        let agents = crowd.get_agents().unwrap();
        assert_eq!(agents.len(), 2);

        crowd.remove_agent(idx0);
        crowd.remove_agent(idx1);

        let count = crowd.agent_count();
        assert_eq!(count, 0);
    }
}
