#import "/src/komet.typ": histogram, histogram-alt

// #assert.eq(
//   histogram((1, 3, 5, 7), bins: (0, 2, 8)),
//   (counts: (1, 3), edges: (0, 2, 8)),
// )


// #assert.eq(
//   histogram(range(10), bins: 3),
//   (counts: (3, 3, 3), edges: (0, 3, 6, 9)),
// )


// #assert.eq(
//   histogram((0, 2, 4), bins: (0, 2, 4)),
//   (counts: (1, 2), edges: (0, 2, 4)),
// )

#histogram(range(20000000), bins: 4)