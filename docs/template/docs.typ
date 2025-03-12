#import "@preview/ilm:1.4.1": *
#import "@preview/zebraw:0.4.6": *

#show: zebraw.with()
#set text(lang: "en")

#show link: it => underline(text(fill: blue)[#it])

#show: ilm.with(
  title: [TiefDownConverter Documentation],
  paper-size: "a4",
  author: "Tiefseetauchner et al",
  date: datetime.today(),
  listing-index: (enabled: true)
)

#include "output.typ"