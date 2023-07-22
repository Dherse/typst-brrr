#import "@preview/tablex:0.0.4": tablex, hlinex, vlinex, colspanx, rowspanx, cellx, gridx

#tablex(
  columns: 4,
  align: center + horizon,
  auto-vlines: false,

  // indicate the first two rows are the header
  // (in case we need to eventually
  // enable repeating the header across pages)
  header-rows: 2,

  // color the last column's cells
  // based on the written number
  map-cells: cell => {
    if cell.x == 3 and cell.y > 1 {
      cell.content = {
        let value = int(cell.content.text)
        let text-color = if value < 10 {
          red.lighten(30%)
        } else if value < 15 {
          yellow.darken(13%)
        } else {
          green
        }
        set text(text-color)
        strong(cell.content)
      }
    }
    cell
  },

  /* --- header --- */
  rowspanx(2)[*Username*], colspanx(2)[*Data*], (), rowspanx(2)[*Score*],
  (),                 [*Location*], [*Height*], (),
  /* -------------- */

  [John], [Second St.], [180 cm], [5],
  [Wally], [Third Av.], [160 cm], [10],
  [Jason], [Some St.], [150 cm], [15],
  [Robert], [123 Av.], [190 cm], [20],
  [Other], [Unknown St.], [170 cm], [25],
)

#tablex(
  columns: 3,
  colspanx(2)[a], (),  [b],
  [c], rowspanx(2)[d], [ed],
  [f], (),             [g]
)

#tablex(
  columns: 3,
  map-hlines: h => (..h, stroke: blue),
  map-vlines: v => (..v, stroke: green + 2pt),
  colspanx(2)[a], (),  [b],
  [c], rowspanx(2)[d], [ed],
  [f], (),             [g]
)

#pagebreak()
#v(80%)

#tablex(
  columns: 4,
  align: center + horizon,
  auto-vlines: false,
  repeat-header: true,

  /* --- header --- */
  rowspanx(2)[*Names*], colspanx(2)[*Properties*], (), rowspanx(2)[*Creators*],
  (),                 [*Type*], [*Size*], (),
  /* -------------- */

  [Machine], [Steel], [5 $"cm"^3$], [John p& Kate],
  [Frog], [Animal], [6 $"cm"^3$], [Robert],
  [Frog], [Animal], [6 $"cm"^3$], [Robert],
  [Frog], [Animal], [6 $"cm"^3$], [Robert],
  [Frog], [Animal], [6 $"cm"^3$], [Robert],
  [Frog], [Animal], [6 $"cm"^3$], [Robert],
  [Frog], [Animal], [6 $"cm"^3$], [Robert],
  [Frog], [Animal], [6 $"cm"^3$], [Rodbert],
)