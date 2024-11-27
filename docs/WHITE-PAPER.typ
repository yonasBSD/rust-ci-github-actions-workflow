#import "@preview/cetz:0.3.1": canvas, draw
#import "@preview/cetz-plot:0.1.0": plot, chart

#set document(title: [Sample White Paper])

#set text(
  font: "Libertinus Serif",
  size: 11pt,
)

#show heading: it => [
  #set align(center)
  #set text(13pt, weight: "regular")
  #block(smallcaps(it.body))
]

#show heading.where(level: 1): it => block(width: 100%)[
  #set align(center)
  #set text(13pt, weight: "regular")
  #smallcaps(it.body)
]

#show heading.where(level: 2): it => text(
  size: 11pt,
  weight: "regular",
  style: "italic",
  it.body + [.],
)

#let title = [
  A fluid dynamic model
  for glacier flow
]

#set page(
  paper: "us-letter",
  header: align(
    right + horizon,
    title,
  ),
  numbering: "1",
  columns: 2,
)

#place(
  top + center,
  float: true,
  scope: "parent",
  clearance: 2em,
)[

  #align(
    center,
    text(17pt)[
      *#title*
    ],
  )

  #grid(
    columns: (1fr, 1fr),
    align(center)[
      Therese Tungsten \
      Artos Institute \
      #link("mailto:tung@artos.edu")
    ],
    align(center)[
      Dr. John Doe \
      Artos Institute \
      #link("mailto:doe@artos.edu")
    ],
  )

  #par(justify: false)[
    *Abstract* \
    #lorem(80)
  ]
]


#outline()

= Introduction
#lorem(150)

= Tables
#table(
  columns: (1fr, auto, auto),
  inset: 10pt,
  align: horizon,
  table.header(
    [],
    [*Volume*],
    [*Parameters*],
  ),

  image("assets/cylinder.svg"),
  $ pi h (D^2 - d^2) / 4 $,
  [
    $h$: height \
    $D$: outer radius \
    $d$: inner radius
  ],

  image("assets/tetrahedron.svg"), $ sqrt(2) / 12 a^3 $, [$a$: edge length],
)
#set table(
  stroke: none,
  gutter: 0.2em,
  fill: (x, y) => if x == 0 or y == 0 {
    gray
  },
  inset: (right: 1.5em),
)

#show table.cell: it => {
  if it.x == 0 or it.y == 0 {
    set text(white)
    strong(it)
  } else if it.body == [] {
    // Replace empty cells with 'N/A'
    pad(..it.inset)[_N/A_]
  } else {
    it
  }
}

#let a = table.cell(
  fill: green.lighten(60%),
)[A]
#let b = table.cell(
  fill: aqua.lighten(60%),
)[B]

#table(
  columns: 4,
  [], [Exam 1], [Exam 2], [Exam 3],
  [John], [], a, [],
  [Mary], [], a, a,
  [Robert], b, a, b,
)

= Figures
#figure(
  image("assets/tiger.jpg"),
  caption: [A nice figure!],
)

= Related Work
#lorem(150)

= Math
$
  sum_(k=0)^n k
  &= 1 + ... + n \
  &= (n(n+1)) / 2
$

= Quotes
#set quote(block: true)
#quote(attribution: [from the Henry Cary literal translation of 1897])[
  ... I seem, then, in just this little thing to be wiser than this man at
  any rate, that what I do not know I do not think I know either.
]

= Symbols
#let envelope = symbol(
  "🖂",
  ("stamped", "🖃"),
  ("stamped.pen", "🖆"),
  ("lightning", "🖄"),
  ("fly", "🖅"),
)

#envelope
#envelope.stamped
#envelope.stamped.pen
#envelope.lightning
#envelope.fly

= Links
#show link: underline
#link("https://example.com")[
  See example.com
]

= References
This was already noted by
pirates long ago. @harry

Multiple sources say ...
@harry @electronic.

#bibliography("assets/works.yml")

Check the docs for more details.
#footnote[https://typst.app/docs]


#set page(width: auto, height: auto, margin: .5cm)

#let data = (
  ([Belgium], 24),
  ([Germany], 31),
  ([Greece], 18),
  ([Spain], 21),
  ([France], 23),
  ([Hungary], 18),
  ([Netherlands], 27),
  ([Romania], 17),
  ([Finland], 26),
  ([Turkey], 13),
)

#align(center)[
  #canvas({
    let colors = gradient.linear(red, blue, green, yellow)

    chart.piechart(
      data,
      value-key: 1,
      label-key: none,
      radius: 4,
      slice-style: colors,
      inner-radius: 1,
      outset: 3,
      inner-label: (content: (value, label) => [#text(white, str(value))], radius: 110%),
      outer-label: (content: "%", radius: 110%))
  })
])


#let data = (
  ([15-24], 18.0, 20.1, 23.0, 17.0),
  ([25-29], 16.3, 17.6, 19.4, 15.3),
  ([30-34], 14.0, 15.3, 13.9, 18.7),
  ([35-44], 35.5, 26.5, 29.4, 25.8),
  ([45-54], 25.0, 20.6, 22.4, 22.0),
  ([55+], 19.9, 18.2, 19.2, 16.4),
)

#canvas({
  draw.set-style(legend: (fill: white))
  chart.barchart(
    mode: "clustered",
    size: (9, auto),
    label-key: 0,
    value-key: (..range(1, 5),),
    bar-width: .8,
    x-tick-step: 2.5,
    data,
    labels: ([Low], [Medium], [High], [Very high]),
    legend: "inner-north-east",
  )
})

#let style = (stroke: black, fill: rgb(0, 0, 200, 75))

#let f1(x) = calc.sin(x)
#let fn = (
  ($ x - x^3"/"3! $, x => x - calc.pow(x, 3) / 6),
  (
    $ x - x^3"/"3! - x^5"/"5! $,
    x => x - calc.pow(x, 3) / 6 + calc.pow(x, 5) / 120,
  ),
  (
    $ x - x^3"/"3! - x^5"/"5! - x^7"/"7! $,
    x => x - calc.pow(x, 3) / 6 + calc.pow(x, 5) / 120 - calc.pow(x, 7) / 5040,
  ),
)

#set text(size: 10pt)

#canvas({
  import draw: *

  // Set-up a thin axis style
  set-style(
    axes: (stroke: .5pt, tick: (stroke: .5pt)),
    legend: (stroke: none, orientation: ttb, item: (spacing: .3), scale: 80%),
  )

  plot.plot(
    size: (12, 8),
    x-tick-step: calc.pi / 2,
    x-format: plot.formats.multiple-of,
    y-tick-step: 2,
    y-min: -2.5,
    y-max: 2.5,
    legend: "inner-north",
    {
      let domain = (-1.1 * calc.pi, +1.1 * calc.pi)

      for ((title, f)) in fn {
        plot.add-fill-between(
          f,
          f1,
          domain: domain,
          style: (stroke: none),
          label: title,
        )
      }
      plot.add(f1, domain: domain, label: $ sin x $, style: (stroke: black))
    },
  )
})
