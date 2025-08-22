#import "plugin.typ": komet-plugin


/// Computes a histogram by counting values that fall into a set of intervals,
/// the so-called bins. 
/// 
/// Returns a dictionary `(counts: array, edges: array)` of histogram counts 
/// and bin edges. 
/// 
/// -> dictionary
#let histogram(
  
  /// An array of float or integer values to compute a histogram from. 
  /// -> array
  values,

  /// A number of bins or an explicit array of bin edges. If $n+1$ bin edges
  /// $e_1,...,e_(n+1)$ are given, the values will be sorted into $n$ bins 
  /// $[e_i, e_(i+1))$ for $i=1,...,n-1$ and $[e_n,e_(n+1)]$. 
  /// -> int | array
  bins: 10,

) = {
  if type(bins) == array {
    bins = bins.map(float).sorted()
  }

  let input = cbor.encode((
    values.map(float),
    bins,
  ))

  cbor(komet-plugin.histogram(input))
}


#let histogram-alt(
  
  /// An array of float or integer values to compute a histogram from. 
  /// -> array
  values,

  /// A number of bins or an explicit array of bin edges. If $n+1$ bin edges
  /// $e_1,...,e_(n+1)$ are given, the values will be sorted into $n$ bins 
  /// $[e_i, e_(i+1))$ for $i=1,...,n-1$ and $[e_n,e_(n+1)]$. 
  /// -> int | array
  bins: 10,

) = {
  let input = values.map(float).map(float.to-bytes.with(size: 8, endian: "big"))

  // let result = komet-plugin.histogram_alt(
  //   bins.to-bytes(size: 8, endian: "big"), 
  //   input
  // )



  // let values = array(result)
  //   .chunks(8)
  //   .map(bytes)
  //   .map(int.from-bytes.with(endian: "big"))

  values
}
