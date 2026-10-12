#import "/src/komet.typ": contour

#assert.eq(
  contour(
    range(4),
    range(4),
    (x, y) => x * y,
    (0, 1),
  ),
  (
    contours: (
      (
        (
          (0.0, 3.0),
          (0.0, 2.0),
          (0.0, 1.0),
          (1.0, 0.0),
          (2.0, 0.0),
          (3.0, 0.0),
        ),
      ),
      (
        (
          (0.33333333333333337, 3.0),
          (0.5, 2.0),
          (1.0, 1.0),
          (2.0, 0.5),
          (3.0, 0.33333333333333337),
        ),
        ((1.0, 1.0), (1.0, 1.0)),
      ),
    ),
    labels: (none, none),
  ),
)

#let labelled = contour(
  range(4),
  range(4),
  (x, y) => x * y,
  (0, 1),
  label-lengths: (1pt, 1pt),
)
#assert.eq(labelled.labels.map(label => label != none), (true, true))
