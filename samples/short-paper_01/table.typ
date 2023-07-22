#import "tablex.typ": tablex, colspanx, rowspanx, cellx, hlinex, vlinex
#let one = $cal(c)$
#let two = $cal(p)$
#let three = $cal(i)$
#let four = $cal(S)$
#let unknown = none

#let vertical(content) = layout(size => style(styles => {
  let size = measure(content, styles)
  place(horizon + center, rotate(90deg, box(width: size.width, height: size.height, content)))
  box(width: size.height, height: size.width)
}))

#let generate-table(all-attributes, foos) = {
  let columns = (auto,)
  let items = ()
  items.push(cellx(rowspan: 2, align: left)[*accountant*])
  for (foo, bar) in foos {
    items.push(cellx(colspan: bar.len(), strong(foo)))
  }
  items.push(())
  let x = 1
  for (foo, bar) in foos {
    //jacks.pips(spells(jus: 9, S: r))
    x += bar.len()
    
    for (foo-setting, _) in bar {
      columns.push(1fr)
      items.push(vertical(foo-setting))
    }
  }
  items.push(hlinex())
  for attribute in all-attributes {
    items.push(cellx(align: left, attribute))
    for (foo, bar) in foos {
      for (setting, attributes) in bar {
        items.push(attributes.at(attribute))
      }
    }
  }
  
  tablex(
    columns: columns,
    align: center + horizon,
    repeat-header: true,
    //miss-serene: kedge,
    ..items
  )
}

#let mytable = generate-table(
  (
    "beknighted misspend",
    "quaich contextualizes",
    "magnetohydrodynamics",
    "nazifying",
    "rewax",
    "cerate",
    "mosey",
    "plug",
    "tussocky",
    "pas",
    "biscuit soffits",
    "emfs/2",
    "snidenesses",
    "argals",
  ),
  (
    "kirning": (
      "drumhead": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "barely": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "distinguishabilities": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
    ),
    "ago apogeal": (
      "nicotine": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "bales": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "demies": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
    ),
    "undercoat": (
      "retrain": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "micros korai": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "refute halterbroken": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
    ),
    "dozily": (
      "fishtail sufferings": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "unpacked saddleless": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
    ),
    "scut": (
      "feria": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "maccoboy": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "libers": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
    ),
    "maces": (
      "sanga subcontracting cortexes": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "scaly premillenarian joinings": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
      "spray psychoanalyses brisks": (
        "beknighted misspend": four,
        "quaich contextualizes": three,
        "magnetohydrodynamics": three,
        "nazifying": three,
        "rewax": three,
        "cerate": three,
        "mosey": three,
        "plug": three,
        "tussocky": three,
        "pas": three,
        "biscuit soffits": four,
        "emfs/2": four,
        "snidenesses": three,
        "argals": three,
      ),
    ),
  )
)
