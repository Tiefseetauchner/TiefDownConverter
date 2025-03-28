#import "@preview/ilm:1.4.1": *
#import "@preview/zebraw:0.4.6": *

#set par(
  first-line-indent: 1em,
  spacing: 0.65em,
)

#set text(font: "Cormorant Garamond")
#show raw: set text(font: ("Iosevka SS10"), size: 9pt)

#show: zebraw.with()

#show link: it => underline(text(fill: blue)[#it])

#show: ilm.with(
  title: [TiefDownConverter Documentation],
  paper-size: "a4",
  author: "Tiefseetauchner et al",
  date: datetime.today(),
  listing-index: (enabled: true),
)

#include "output.typ"
