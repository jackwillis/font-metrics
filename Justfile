cpp font_name font_size="12" text_width="36":
    rm -f cpp_vars.aux
    >>cpp_vars.aux printf "\\def%sargfontname{%s}" "\\" "{{font_name}}"
    >>cpp_vars.aux printf "\\def%sargfontsize{%s}" "\\" "{{font_size}}pt"
    >>cpp_vars.aux printf "\\def%sargtextwidth{%s}" "\\" "{{text_width}}pc"

    xelatex -quiet -interaction=nonstopmode -jobname=cpp "\input{cpp_vars.aux}\input{cpp}"

    cargo run --bin cpp -- "cpp.pdf" --width {{text_width}}

    rm -f cpp_vars.aux cpp.aux cpp.log cpp.pdf

xheight file_name:
    cargo run --quiet --bin xheight -- "{{file_name}}"