#import "@preview/ilm:1.4.1": *
#import "@preview/zebraw:0.4.6": *

#import "./metadata.typ": meta

#set par(
  first-line-indent: 1em,
  spacing: 0.65em,
)

#show: zebraw.with()
#set text(lang: "en")

#show link: it => underline(text(fill: blue)[#it])

#show: ilm.with(
  title: [#meta.title],
  paper-size: "a4",
  author: meta.author,
  date: datetime.today(),
  listing-index: (enabled: true)
)

#include "output.typ"