#let wordColor = black
#let wordSize = 1.1em

#let explainColor = black

#let strokeColor = maroon
#let strokeFont = "Roboto Mono"
#let strokeWeight = "bold"
#let strokeSize = 1.1em

#let stenoSize = 8em
#let stenoFill = luma(250)
#let stenoStroke = 0.2pt  + blue

#let stroke(stroke) = {
  text(weight: strokeWeight, font: strokeFont, size: strokeSize, fill: strokeColor)[#stroke]
}

#let steno(word, strokeText) = {
  box(fill: stenoFill, stroke: stenoStroke, radius: 0.5em, inset: 0.5em)[
    #align(center)[#text(fill: wordColor, size: wordSize)[#strong(word)]]
    #text(font: "Stenodisplay Classic", size: stenoSize)[#strokeText]\
    #stroke([#align(center)[#strokeText]])  
  ]
  h(10pt)
}

#let stenoExplain(word, strokeText, explanation) = {
  box(fill: stenoFill, stroke: stenoStroke, radius: 0.5em, inset: 0.5em)[
    #align(center)[#text(fill: wordColor, size: wordSize)[#strong(word)]]
    #align(center)[#text(fill: explainColor)[#explanation]]
    #text(font: "Stenodisplay Classic", size: stenoSize)[#strokeText]\
    #stroke([#align(center)[#strokeText]])
  ]
  h(10pt)
}

#let todo(msg) = {
  [#text(fill: red, weight: "bold", size: 12pt)[TODO: #msg]]
}
