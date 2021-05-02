(define-module (qrxfil)
  #:use-module (guix packages)
  #:use-module (guix git-download)
  #:use-module (guix build-system cargo)
  #:use-module (gnu packages crates-io)
  #:use-module (gnu packages crates-graphics)
  #:use-module (qrxfil-deps)
  #:use-module ((guix licenses) #:prefix license:))

(define-public qrxfil
  (package
    (name "qrxfil")
    (version "0.3.1")
    (source
     (origin
       (method git-fetch)
       (uri (git-reference
	     (url "https://github.com/OverkillGuy/qrxfil")
	     (commit (string-append "v" version))))
       (file-name (git-file-name name version))
       (sha256 "0dq4izy4dsm6hkl7iy5nd9ka6pn010nbl7n08rhvhniclli8ksz1")))
    (build-system cargo-build-system)
    (arguments
     `(#:tests? #f                  ; missing test deps
       #:cargo-inputs
       (
	("rust-qrcode" ,rust-qrcode-0.12)
	("rust-qr-code" ,rust-qr-code-1)
	("rust-image" ,rust-image-0.23)
	("rust-base64" ,rust-base64-0.13)
	("rust-clap" ,rust-clap-2)
	("rust-itertools" ,rust-itertools-0.10)
	("rust-thiserror" ,rust-thiserror-1-0-24)
	("rust-pandoc" ,rust-pandoc-0.8)
	("rust-csv" ,rust-csv-1))
       #:cargo-development-inputs
       (("rust-assert-fs" ,rust-assert-fs-1)
	("rust-pdf" ,rust-pdf-0.7)
	("rust-rqrr" ,rust-rqrr-0.3)
	("rust-test-case" ,rust-test-case-1-1))))
    (home-page "https://github.com/OverkillGuy/qrxfil")
    (synopsis "Exfiltrate files via QR codes")
    (description
     "Exfiltrate files via QR codes.

Splitting files across multiple QR codes for “sending” across air-gapped computer systems. Generates numbered PNG files to scan, can save PDF.

The codes contain metadata about chunk number (e.g. “007 of 103”) to enable out-of-order scanning and reconstruction.")
    (license license:gpl3+)))
