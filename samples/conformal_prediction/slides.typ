#let NO_STEPS = false

#let slide_counter = state("slide_counter", 1)

#let slide_inner(n_steps, content) = {
    if n_steps >= 20 [
        // for some bizzare reason we need this in order to not enter an infinite loop
    ] else [
        #slide_counter.update(n_steps)
        #box(width: 100%, height: 100%)[#content]
        #pagebreak()
        #slide_counter.display(x => if x < 1 [#slide_inner(n_steps + 1, content)] else [])
    ]
}
#let slide(content) = {
  if NO_STEPS {
    slide_inner(19, content)
  } else {
    slide_inner(1, content)
  }
}

#let step(rest) = {
    slide_counter.update(k => k - 1)
    slide_counter.display(x => [
        // #place([#text(fill: orange)[#x]])
        #if x > 0 {
            rest
        } else {
            hide(rest)
        }
    ])
}
#let steplist(list, spread: true, start: 1) = [
    #let i = 0
    #if spread [ #v(1fr) ]
    #for item in list.children.filter(x => x != [ ]) [
        #if i >= start [
            #show: step
            #item
        ] else [
            #item
        ]
        #if spread [ #v(1fr) ]
        #{ i += 1 }
    ]
    // #columns(2)[#text(size: 8pt)[#repr(list.children)]]
]

#let slide_deck(doc, page_format: "16-9", margin: 2em) = [
    #set page("presentation-" + page_format, margin: margin)

    #show heading.where(level: 1): it => text(50pt, weight: "bold")[#it]
    #show heading.where(level: 2): it => text(42pt, weight: "bold")[#it]
    #show heading.where(level: 3): it => text(34pt, weight: "bold")[#it]
    #set text(24pt)

    // #let distribute = it => [#v(1fr) it]
    // #show par: distribute
    // #show listitem: distribute

    #doc
]
