//! Market and price resolving.
//! 
//! Structs containing information about state of the market, pricing and 
//! goods flow, underlying infrastructure. Includes algorithms used to
//! calculate prices.
//! 
//! Price resolving algorith abstract:
//! Special graph is generated then max value flow is computed. Computed flow 
//! represents used consumption, demand and goods transfers.