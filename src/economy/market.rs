//! Market and price resolving.
//! 
//! Structs containing information about state of the market, pricing and 
//! goods flow, underlying infrastructure. Includes algorithms used to
//! calculate prices.
//! 
//! ## Algorithm
//! 
//! ### Abstract
//! 
//! Special graph is generated then max value flow is computed. Computed flow 
//! represents used consumption, demand and goods transfers. Then consumers and
//! producers update their prices and algorithm repeats.
//! 
//! 
//! ### Full
//! 
//! At the beggining all entities calculate their supply/demand as if they did
//! not have any market power. Then algorithms consists of multiple rounds of
//! calculating new prices and entities updating their supply/demand.
//! 
//! #### Calculating prices
//! 
//! Special directed graph, where each edge has it's capacity and value, is
//! constructed:
//! 1. Graph of city connections is taken. Each edge has value equal to 
//!     minus transport cost.
//! 2. Node is added for each entity. 
//! 3. Edges from entity nodes to cities are added in such a way that they 
//!     represent their costs/willingness to pay.
//! Then maximum value flow is found. Flow on edge from city to entity 
//! corresponds to their production/consumption and flow on city to city edge
//! represents transport between towns.
//! 
//! #### Ending algorithm
//! 
//! Each entity is expected to lower their demand/supply untill they reach
//! optimal value. Therefore algorithm should stop once changes in prices
//! are small enough.
//! 