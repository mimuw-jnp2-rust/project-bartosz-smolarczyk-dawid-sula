//! Representation of generic market entity.
//! 
//! ## Basic algorithm:
//! 
//! Producer creates goods untill price is equal to marginal costs. Requires 
//! only one iteration of the algorithm.
//! 
//! ## Regular algorithm: 
//! 
//! Producer produces according to basic algorithm then in each iteration he
//! corrects his production based on marginal costs. Single producer can only
//! directly supply only one node in the graph.
//! 
//! ## Advanced algorithm:
//! 
//! Like regular algorithm, but we allow supplying multiple nodes.
//! Producer separates nodes they supply into groups where prices are strongly
//! connected. They reduce sales in different groups as long as it's beneficial
//! to them. Changing prices can cause changes to where goods are transported
//! therefore events for each possible change are created and they are handled
//! when they appear.
//! 
//! Note: This description is still unclear but it's enough for me to know
//! what to do.