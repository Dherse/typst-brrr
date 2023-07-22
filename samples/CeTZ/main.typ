#import "@preview/cetz:0.0.1"

#set page(width: auto, height: auto)

#show math.equation: block.with(fill: white, inset: 1pt)

#cetz.canvas(length: 3cm, {
  import cetz.draw: *

  set-style(
    mark: (fill: black),
    stroke: (thickness: 0.4pt, cap: "round"),
    arc: (
      radius: 0.3,
      mode: "PIE",
      fill: green.lighten(80%),
      stroke: (paint: green.darken(50%))
    ),
    content: (padding: 1pt)
  )

  grid((-1.5, -1.5), (1.4, 1.4), step: 0.5, stroke: gray + 0.2pt)

  circle((0,0), radius: 1)

  line((-1.5, 0), (1.5, 0), mark: (end: ">"))
  content((), $ x $, anchor: "left")
  line((0, -1.5), (0, 1.5), mark: (end: ">"))
  content((), $ y $, anchor: "bottom")

  for (x, ct) in ((-1, $ -1 $), (-0.5, $ -1/2 $), (1, $ 1 $)) {
    line((x, 3pt), (x, -3pt))
    content((), anchor: "above", ct)
  }

  for (y, ct) in ((-1, $ -1 $), (-0.5, $ -1/2 $), (0.5, $ 1/2 $), (1, $ 1 $)) {
    line((3pt, y), (-3pt, y))
    content((), anchor: "right", ct)
  }

  // Draw the green angle
  arc((0,0), start: 0deg, stop: 30deg, anchor: "origin", name: "arc")
  content((15deg + 4deg, 0.2), text(green)[#sym.alpha])

  line((0,0), (1, calc.tan(30deg)))

  set-style(stroke: (thickness: 1.2pt))

  line((30deg, 1), ((), "|-", (0,0)), stroke: (paint: red), name: "sin")
  content("sin", text(red)[$ sin alpha $], anchor: "right")
  line("sin.end", (0,0), stroke: (paint: blue), name: "cos")
  content("cos", text(blue)[$ cos alpha $], anchor: "top")
  line((1, 0), (1, calc.tan(30deg)), name: "tan", stroke: (paint: orange))
  content("tan", $ text(#orange, tan alpha) = text(#red, sin alpha) / text(#blue, cos alpha) $, anchor: "left")
})

#cetz.canvas({
  import cetz.draw: *

  let chart(..values, name: none) = {
    let values = values.pos()

    let offset = 0
    let total = values.fold(0, (s, v) => s + v.at(0))

    let segment(from, to) = {
      merge-path(close: true, {
        line((0, 0), (rel: (-360deg * from, 1)))
        arc((), start: from * -360deg, stop: to * -360deg, radius: 1)
      })
    }

    group(name: name, {
      stroke((paint: black, join: "round"))

      let i = 0
      for v in values {
        fill(v.at(1))
        let value = v.at(0) / total

        // Draw the segment
        segment(offset, offset + value)

        // Place an anchor for each segment
        anchor(v.at(2), (offset * -360deg + value * -180deg, .75))

        offset += value
      } 
    })
  }

  // Draw the chart
  chart((10, red, "red"),
        (3, blue, "blue"),
        (1, green, "green"),
        name: "chart")
  
  set-style(mark: (fill: white, start: "o", stroke: black),
            content: (padding: .1))

  // Draw annotations
  line("chart.red", ((), "-|", (2, 0)))
  content((), [Red], anchor: "left")

  line("chart.blue", (1, 1), ((), "-|", (2,0)))
  content((), [Blue], anchor: "left")

  line("chart.green", ((), "-|", (2,0)))
  content((), [Green], anchor: "left")
})