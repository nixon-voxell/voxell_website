#let red = rgb("#FF6188")
#let orange = rgb("#FC9867")
#let yellow = rgb("#FFD866")
#let green = rgb("#A9DC76")
#let blue = rgb("#78DCE8")
#let purple = rgb("#AB9DF2")
#let base0 = rgb("#19181A")
#let base1 = rgb("#221F22")
#let base2 = rgb("#2D2A2E")
#let base3 = rgb("#403E41")
#let base4 = rgb("#5B595C")
#let base5 = rgb("#727072")
#let base6 = rgb("#939293")
#let base7 = rgb("#C1C0C0")
#let base8 = rgb("#FCFCFA")

#let title(body) = {
  layout(size => {
    set text(
      fill: base7,
      size: size.height * 0.3,
      font: "Consolas",
    )
    set align(center + horizon)

    box(
      width: 100%,
      height: 100%,
    )[= #body]
  })
}

#let button(body) = {
  layout(size => {
    let min_size = calc.min(size.width, size.height)
    set text(fill: base0, size: min_size * 0.3)
    set align(center + horizon)

    box(
      width: 100%,
      height: 100%,
      fill: blue.transparentize(60%),
      stroke: blue + 0.2em,
      radius: 1em,
    )[= #body]
  })
}
