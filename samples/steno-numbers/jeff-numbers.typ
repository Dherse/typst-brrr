#import "steno.typ" :*

#let title = "Jeff-Numbers — Cheat Sheet"
#let author = "Yann Cebron"
#let version = "0.1"

#let grid = 10pt

#let accentColor = blue
#let accentTextColor = white

#let headerColor = blue
#let headerLighten = 20%

#set document(author: author, title: title)
#set text(font: "Roboto") 

#show link: underline

#show heading.where(level:1): it => [
  #set align(center)
  #set text(fill: accentTextColor)
  #block(fill: accentColor.darken(headerLighten), width: 100%, outset: grid, smallcaps(it.body))
  #v(grid)
]

#show heading.where(level:2): it => [
  #v(grid)
  #set align(left)
  #set text(fill: accentTextColor)
  #block(fill: headerColor.lighten(headerLighten), width: 100%, outset: grid, it.body)
  #v(grid)
]

#set page(
  width: 47cm, height: 58cm, 
  columns: 2, numbering: "1", margin: (top:105pt, x: grid*2, bottom: 25pt), 
  header: [
    #set text(fill: accentTextColor)
    #rect(fill: accentColor, width: 100%, inset: grid, outset: (left:100pt, right:100pt))[
      
      #text(size: 2em, weight: "bold")[#title #counter(page).display("I")]
      
      #text(size: 1.5em)[#("Enhanced number system for Plover")] #h(grid) #text(size: 1.2em)[https://github.com/jthlim/jeff-numbers]
    ]
  ],
  footer: [
    #rect(fill: accentColor, width: 100%, inset: grid*0.5, outset: (left:100pt, right:100pt))[
    #set text(fill: accentTextColor)
    #align(right)[v#version #sym.floral Made with #link("https://typst.app")[typst] by #link("https://www.yanncebron.com")[#author] #sym.floral Steno-Font by #link("https://github.com/Kaoffie/steno_font")[Kaoffie]]
   ]
  ]
)



#block(stroke: 0.5pt + accentColor.darken(headerLighten), inset: grid)[
  
= Input Modifiers

== Reverse
#stroke("EU"), #stroke("E") (for Multisteno) or #stroke("U") (for Uni) reverses any stroke, and works with any number of digits. 

#stenoExplain("42", "24EU", "24 (Reverse)")
#stenoExplain("97031", "130EU79", "13079 (Reverse)")

== Double Last Digit
#stroke("-D") will always double the last digit.

#stenoExplain("11", "1-D", "1 (Double Last Digit [1])")
#stenoExplain("122", "12-D", "12 (Double Last Digit [2])")
#stenoExplain("3211", "123EUD", "123 (Reverse) (Double Last Digit [1])")

== Roman Numerals
#stroke("R") or #stroke("-R") converts a number to roman numerals, #stroke("*R") for lower case. 
// todo R + -R ??
This will only work for numbers between 1 and 3999 inclusive.
 
#stenoExplain("XII", "12R", "12 (Roman Numerals)")
#stenoExplain("MCMXCII", "19/2EUR9", "19 / 29 (Reverse) (Roman Numerals) ") // todo

]


//#v(3em)
#block(stroke: 0.5pt + accentColor, inset: grid)[
= Separators

== Decimal Point
#stroke("*") will add a decimal point after.

#stenoExplain("12.34", "12*/34", "12 (Decimal Point) / 34")

== Comma
#stroke("*S") will add a comma after.

#stenoExplain("12,340.50", "12*S/340*/50", "12 (Comma) / 340 (Decimal Point) / 50")

]


#colbreak()
#block(stroke: 0.5pt + accentColor, inset: grid)[
= Suffixes

== 00 --- Hundred Suffix
#stroke("-Z") will add the suffix _00_.

#stenoExplain("200", "2-Z", "2 (Hundred Suffix)") 
#stenoExplain("2300", "2/3-Z", "2 / 3 (Hundred Suffix)") 

== ,000 --- Thousand Suffix
#stroke("*Z") will add the suffix _,000_.

#stenoExplain("12,000", "12*Z", "12 (Thousand Suffix)")
#stenoExplain("12,000,000", "12*Z/#*Z", "12 (Thousand Suffix) / (Thousand Suffix)")

== % --- Percent Symbol Suffix
#stroke("KR-") or #stroke("-RG") will suffix the entire number with a percent symbol.

#stenoExplain("79%", "KR-79", "79 (Percent Symbol Suffix)")
#stenoExplain("23%", "23-RG", "23 (Percent Symbol Suffix)")
#stenoExplain("112%", "1/12-RG", "1 / 12 (Percent Symbol Suffix)")
#stenoExplain("100%", "1KR-Z", "1 (Hundred Suffix) (Percent Symbol Suffix)")

== n#super("th") --- Ordinal Suffix
#stroke("W") or #stroke("-B") will add ordinal suffixes.

#stenoExplain("1st", "1-B", "1 (Ordinal Suffix)")
#stenoExplain("21st", "2/1-B", "2 / 1 (Ordinal Suffix)")

#stenoExplain("7th", "W-7", "7 (Ordinal Suffix)")
#stenoExplain("11th", "1/1-B", "1 / 1 (Ordinal Suffix)")

]


#colbreak()
#block(stroke: 0.5pt + accentColor, inset: grid)[
= Currency

== Dollar Value
#stroke("WR-") or #stroke("-RB") will format the entire number as a dollar value.
If the value ends with a decimal point, _.00_ will be appended.

#stenoExplain("$7", "WR-7", "7 (Dollar Value)")
#stenoExplain("$23", "23-RB", "23 (Dollar Value)")
#stenoExplain("$1,234", "1234-RB", "1234 (Dollar Value)")

#stenoExplain("$1,234.00", "1234*RB", "1234 (Decimal Point) (Dollar Value)")
#stenoExplain("$1,234.50", "1234*/50-RB", "1234 (Decimal Point) / 50 (Dollar Value)")

== Hundreds of Dollars
#stroke("-DZ") will convert a number to hundreds of dollars, and works with multiple strokes. 

#stenoExplain("$100", "1-DZ", "1 (Hundreds of Dollars)")
#stenoExplain("$1,200", "1/2-DZ", "1 / 2 (Hundreds of Dollars)")

]


#block(stroke: 0.5pt + accentColor, inset: grid)[
= Clock

== Full Hour
#stroke("K-") or #stroke("-BG") will add the suffix _:00_.

#stenoExplain("9:00", "K-9", "9 (Full Hour)")
#stenoExplain("2:00", "2-BG", "2 (Full Hour)")
#stenoExplain("11:00", "1-BGD", "1 (Double Last Digit [1]) (Full Hour)")

== 15-Minute Steps
Using #stroke("K") combined with #stroke("-B") and/or #stroke("-G") gives 15 minute increments.

#stenoExplain("7:15", "K-7G", "7 (:15 Suffix)")
#stenoExplain("9:30","K-B9", "9 (:30 Suffix)")
#stenoExplain("3:45", "3K-BG", "3 (:45 Suffix)")


== AM/PM Suffix
Adding #stroke("-S") or #stroke("*S") will add suffix _a.m._ or _p.m._

#stenoExplain("3:00 a.m.", "3-BGS", "3 (Full Hour) (AM Suffix)")
#stenoExplain("12:00 p.m.", "12/#K*-S", "12 / (Full Hour) (PM Suffix)")

]


#colbreak()
#block(stroke: 0.5pt + accentColor, inset: grid)[
= Words

== To Words
#stroke("-G") will convert the number to words.

#stenoExplain("twelve", "12-G", "12 (To Words)")
#stenoExplain("two thousand", "2*GZ", "2 (Thousand Suffix) (To Words)")

#stenoExplain("twelve million", "12*Z/#*GZ", "12 (Thousand Suffix) / (Thousand Suffix) (To Words)")
   
Note that #stroke("-S") suffix with Plover's orthography rules will work as expected.
      
#stenoExplain("thirties", "30GS", "30 (To Words) (Pluralize)")
#stenoExplain("forty-fours", "4-D/#-GS", "4 (Double Last Digit [4]) / (To Words) (Pluralize)")

== Ordinal Words        
Can be combined with #stroke("W-") to give ordinal words.

#stenoExplain("twenty-first", "12WEUG", "12 (Reverse) (Ordinal Word)")
#stenoExplain("tenths", "1W0GS", "10 (Ordinal Word)")

This can also be done as a suffix stroke.

#stenoExplain("one hundredth", "1-Z/#W-G", "1 (Hundred Suffix) / (Ordinal Word)")

]

#v(30em)


= Support the #link("http://www.openstenoproject.org/")[Open Steno Project]