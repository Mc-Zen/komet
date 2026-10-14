#import "plugin.typ": komet-plugin



/// Generates contours from intersecting a function on a 2d rectangular mesh
/// with planes parallel to the z-plane at `z=level`.
///
/// The `z` argument can either be a 
/// - two-dimensional `m×n` array where `m` matches the number of y-values
///   and `n` matches the number of x-values. 
/// - or a function that takes an `x` and a `y` value and returns a 
///   corresponding `z` coordinate. 
/// 
/// The input x and y arrays need to be monotonically sorted, either ascending
/// or descending.
/// 
/// The return value is a dictionary with the keys
/// - `contours`: An array of contours for each level
///   - where each contour consists of an array of contour lines (there can be 
///     more than one disjoint curve per level), 
///   - where each contour line comprises a set of vertices `(x, y)` making up the curve.
/// - `labels`: An array of label positions for each level. Each label is either
///   `none` (if the corresponding input label length was `0pt` or no suitable
///   location could be found) or a dictionary containing the label's `x` and 
///   `y` coordinates and its `angle`.
/// 
/// -> dictionary
#let contour(

  /// A sorted one-dimensional array of $x$ data coordinates. 
  /// -> array
  x, 
  
  /// A sorted one-dimensional array of $y$ data coordinates. 
  /// -> array
  y, 

  /// Specifies the z coordinates (heights). 
  /// -> array | function
  z, 

  /// Specifies the levels to compute contours for. 
  /// -> int | float | array. 
  levels,

  /// Optional lengths for contour labels. This should either be none or an
  /// array with the same length as @contour.levels. If given, optimal positions
  /// and angles for labels are computed and the output curves will be split to
  /// accommodate for the labels. Array elements should be of type `length` with
  /// only a pt part. For lengths that are equal to `0pt`, no label is assumed.
  /// -> none | array
  label-lengths: none

) = {
  if type(z) == function {
    z = y.map(y => x.map(x => z(x, y)))
  }

  if type(levels) in (int, float) {
    levels = (levels,)
  }

  if label-lengths == none {
    label-lengths = (0pt, ) * levels.len()
  }
  assert(
    label-lengths.len() == levels.len(),
    message: "The number of label-lengths does not match the number of levels"
  )
  let flattened-z = z.flatten()
  assert(
    x.len() * y.len() == flattened-z.len()
  )

  let data = cbor.encode((
    x.map(float),
    y.map(float),
    z.flatten().map(float),
    levels.map(float),
    label-lengths.map(length => length.pt())
  ))
  let result = cbor(komet-plugin.contour(data))
  let labels = result.labels.map(label =>
    if label.len() == 0 {
      none
    } else {
      (x: label.at(0), y: label.at(1), angle: label.at(2) * 1deg)
    }
  )
  (contours: result.contours, labels: labels)
}
