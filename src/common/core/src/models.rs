use serde::{Serialize, Deserialize} ;

#[derive(Serialize, Deserialize,Debug , Clone)]
pub enum ExperimentState {
    Draft,
    LiveApprovalPending,
    LiveApproved,
    Live,
    StopApprovalPending,
    Stopped
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Experiment {
    name : String,
    objective : Option<String>,
    state :ExperimentState
}