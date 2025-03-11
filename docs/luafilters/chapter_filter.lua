function Header(el)
    if el.level == 1 then
        return {
            pandoc.RawBlock("latex", "\\newpage{}"), -- Page break for LaTeX
            el
        }
    end
end
