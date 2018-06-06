cpp-latex-build font_name font_size="12pt" text_width="36pc":
    rm -f cpp_vars.aux
    >>cpp_vars.aux printf "\\def%sargfontname{%s}" "\\" "{{font_name}}"
    >>cpp_vars.aux printf "\\def%sargfontsize{%s}" "\\" "{{font_size}}"
    >>cpp_vars.aux printf "\\def%sargtextwidth{%s}" "\\" "{{text_width}}"
    xelatex -interaction=nonstopmode -jobname=cpp "\input{cpp_vars.aux}\input{cpp}"

cpp-latex-clean:
    rm -f cpp_vars.aux cpp.aux cpp.log cpp.pdf

xheight file_name:
    cargo run --quiet --bin xheight -- "{{file_name}}"