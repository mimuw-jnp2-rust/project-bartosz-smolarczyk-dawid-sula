//! Basic algorithm:
//! Producer creates goods untill price is equal to marginal costs. Only r
//! equires one iteration of the algorithm.
//! 
//! Regular algorithm: 
//! Producer produces according to basic algorithm then in each iteration he
//! corrects his production based on marginal costs. Single producer can only
//! directly supply one node in the graph.
//! 
//! Advanced algorithm:
//! Like regular algorithm, but graphs of edges where goods are transported
//! are used to allow supplying multiple nodes.