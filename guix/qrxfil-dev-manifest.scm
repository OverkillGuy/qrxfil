;; Enough to run "make check build"
(specifications->manifest
 '("rust"
   "rust:cargo"
   "rust:rustfmt"
   ; Missing target in guix: clippy (rust linter)
   "rust-clippy"
   "make"
   "python-pre-commit"))
;; Extras to run full "make test" (>2GB pkgs)
; "texlive" "pandoc"
