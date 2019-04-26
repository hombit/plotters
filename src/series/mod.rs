/*!
  This module contains predefined types of series. 
  The series in Plotters is actually an iterator of elements, which 
  can be taken by `ChartContext::draw_series` function. 
  
  This module defines some "iterator transformer", which transform the data
  iterator to the element iterator.

  Any type that implements interator emitting drawable elements are acceptable series.
  So iterator combinator such as `map`, `zip`, etc can also be used.
*/

mod point_series;
mod line_series;

pub use point_series::PointSeries;
pub use line_series::LineSeries;
