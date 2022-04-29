/// Demand and supply functions.
/// 
/// Common tools used to model economic functions and constructs representing 
/// demand, supply and their difference.
mod function;

/// Producers and consumers.
/// 
/// Structs representing consumers and producers and their ways of interacting
/// with the market.
mod entity;

/// Cities and connections between them.
/// 
/// Structs representing cities on the market, ways they are connected, 
/// transport costs and networks capacity limits.
mod geography;

/// Market and price resolving.
/// 
/// Structs containing information about state of the market, pricing and 
/// goods flow, underlying infrastructure. Includes algorithms used to
/// calculate prices.
mod market;