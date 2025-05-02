#import "@preview/touying:0.6.1": *
#import "@preview/numbly:0.1.0": numbly
#import themes.simple: *

#let france_red = "e1000f"
#let france_blue = "000091"
#let palette_primary_01 = "FF9575"
#let palette_primary_02 = "CE614A"
#let palette_primary_03 = "AD4847"
#let palette_secondary_01 = "FFE552"
#let palette_secondary_02 = "C8AA39"
#let palette_secondary_03 = "716043"

#show: simple-theme.with(
  header: self => [
    #set text(fill: self.colors.neutral-darkest, size: 12pt, font: "Spectral")
    #v(2em)

    #box(height: auto, width: auto, inset: 0.5em, radius: 0.5cm, align(
      top,
    )[#stack(
        dir: ltr,
        image("republique_française.png", height: 4.5em),
        h(1fr),
        utils.display-current-short-heading(),
      )])
  ],
  footer: self => [
    #set text(fill: self.colors.neutral-darkest, size: 12pt, font: "Spectral")
    #box(height: auto, width: auto, inset: 0.5em, radius: 0.5cm, stack(
      dir: ttb,
      line(length: 100%),
      v(1em),
      stack(
        dir: ltr,
        [Direction Générale des Finances Publiques],
        h(1fr),
        context utils.slide-counter.display(),
        h(1em),
        datetime.today().display(self.datetime-format),
      ),
    ))
  ],
  footer-right: none,
  config-colors(
    primary: rgb(palette_primary_01),
    primary-dark: rgb(palette_primary_01).darken(20%),
    primary-darker: rgb(palette_primary_01).darken(50%),
    secondary: rgb(palette_primary_02),
    secondary-dark: rgb(palette_primary_02).darken(20%),
    secondary-darker: rgb(palette_primary_02).darken(50%),
    tertiary: rgb(palette_primary_03),
    tertiary-dark: rgb(palette_primary_03).darken(20%),
    tertiary-darker: rgb(palette_primary_03).darken(50%),
    neutral: rgb(palette_secondary_01),
    neutral-darker: rgb(palette_secondary_01).darken(20%),
    neutral-darkest: rgb(palette_secondary_01).darken(50%),
    neutral-light: rgb(palette_secondary_01).lighten(20%),
    neutral-lightest: rgb(palette_secondary_01).lighten(50%),
  ),
  config-methods(init: (self: none, body) => {
    set text(fill: self.colors.primary-dark, size: 20pt, font: "Marianne")
    show footnote.entry: set text(size: .6em)
    show strong: self.methods.alert.with(self: self)
    show heading.where(level: self.slide-level + 1): set text(1.4em)
    set par(justify: true)
    body
  }),
  config-common(datetime-format: "[day]/[month]/[year]"),
  config-page(margin: (
    top: 5.5em,
    right: 2em,
    left: 2em,
    bottom: 3em,
  )),
  aspect-ratio: "16-9",
)

#title-slide[
  #set page(
    margin: (
      top: 8em,
      right: 2em,
      left: 2em,
      bottom: 3em,
    ),
    header: [
      #box(height: auto, width: auto, inset: 0.5em, radius: 0.5cm, align(top)[
        #stack(
          dir: ltr,
          image("republique_française.png", height: 4em),
          h(1fr),
          image("dgfip.svg", height: 3em),
        )
      ])
    ],
  )

  = SharedAgenda
  == Votre agenda dans le Cloud!
]
== Introduction
=== But

---

=== Solution

---

=== Architecture

---

=== Infrastructure (V1)

---

=== Infrastructure (V2)
== Server
=== Choix Techniques
#box(inset: 0.5em, radius: 0.5cm, width: auto, height: auto, align(center)[
  #stack(
    dir: ltr,
    image("rust-logo-blk.svg", width: 20%),
    image("rest-api-icon.svg", width: 20%),
    image("rocket-svgrepo-com.svg", width: 20%),
  )
])
---

=== Implémentation

---

=== Difficultées Rencontrées

== Client (CLI/REPL)
=== Choix Techniques

---

=== Implémentation

---

=== Démonstration
