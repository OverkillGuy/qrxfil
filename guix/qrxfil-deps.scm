+;;; Dependencies required to install qrxfil, that aren't already inside guix
(define-module (qrxfil-deps)
  #:use-module (guix packages)
  #:use-module (guix download)
  #:use-module (guix build-system cargo)
  #:use-module (gnu packages crates-io)
  #:use-module (gnu packages crates-graphics)
  #:use-module ((guix licenses) #:prefix license:))

;; qrcode rust package
(define-public rust-checked-int-cast-1
  (package
    (name "rust-checked-int-cast")
    (version "1.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "checked_int_cast" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "06brva5agm6g12q15f8fidz17akb85q211496p1k2qxhb9mmxk0p"))))
    (build-system cargo-build-system)
    (arguments `(#:skip-build? #t))
    (home-page
     "https://github.com/PeterReid/checked_int_cast")
    (synopsis
     "Conversions between primitive integers with overflow and underflow checking
")
    (description
     "Conversions between primitive integers with overflow and underflow checking
")
    (license license:expat)))

(define-public rust-qrcode-0.12
  (package
    (name "rust-qrcode")
    (version "0.12.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "qrcode" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "0zzmrwb44r17zn0hkpin0yldwxjdwya2nkvv23jwcc1nbx2z3lhn"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs
       (("rust-checked-int-cast"
	 ,rust-checked-int-cast-1)
	("rust-image" ,rust-image-0.23))
       #:cargo-development-inputs
       (("rust-image" ,rust-image-0.23))))
    (home-page
     "https://github.com/kennytm/qrcode-rust")
    (synopsis "QR code encoder in Rust")
    (description "QR code encoder in Rust")
    (license (list license:expat license:asl2.0))))

;; qr_code rust package
(define-public rust-g2poly-0.4
  (package
    (name "rust-g2poly")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "g2poly" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "0ys5r96dr6ywam37mxkhj6wbp9qd6l5hxjc9gvq0g9gwi1w7cdz8"))))
    (build-system cargo-build-system)
    (arguments `(#:skip-build? #t))
    (home-page "https://github.com/WanzenBug/g2p")
    (synopsis
     "Primitive implementation of polynomials over the field GF(2)
")
    (description
     "Primitive implementation of polynomials over the field GF(2)
")
    (license (list license:expat license:asl2.0))))

(define-public rust-g2gen-0.4
  (package
    (name "rust-g2gen")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "g2gen" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "0m8590wqm57p4dik77s5x8lcydas9kwkzllc71f8r033djqh1h9g"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build?
       #t
       #:cargo-inputs
       (("rust-g2poly" ,rust-g2poly-0.4)
	("rust-proc-macro2" ,rust-proc-macro2-0.4)
	("rust-quote" ,rust-quote-0.6)
	("rust-syn" ,rust-syn-0.15))))
    (home-page "https://github.com/WanzenBug/g2p")
    (synopsis
     "A macro to create types that implement fast finite field arithmetic.
")
    (description
     "This package provides a macro to create types that implement fast finite field arithmetic.
")
    (license (list license:expat license:asl2.0))))

(define-public rust-g2p-0.4
  (package
    (name "rust-g2p")
    (version "0.4.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "g2p" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "0m97qvalz4aq68hps0m19gkh617qr2y4a3ik1d1fbjr94rivq2dz"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build?
       #t
       #:cargo-inputs
       (("rust-g2gen" ,rust-g2gen-0.4)
	("rust-g2poly" ,rust-g2poly-0.4))))
    (home-page "https://github.com/WanzenBug/g2p")
    (synopsis
     "A crate to create types that implement fast finite field arithmetic.
")
    (description
     "This package provides a crate to create types that implement fast finite field arithmetic.
")
    (license (list license:expat license:asl2.0))))

(define-public rust-bmp-monochrome-1
  (package
    (name "rust-bmp-monochrome")
    (version "1.0.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "bmp-monochrome" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "1hc0mlmy26jpvx34rw896ki60wkb6c2finnsfs3zvzfin5bwpmi0"))))
    (build-system cargo-build-system)
    (arguments
     `(#:skip-build?
       #t
       #:cargo-inputs
       (("rust-arbitrary" ,rust-arbitrary-0.4)
	("rust-image" ,rust-image-0.23))))
    (home-page
     "https://github.com/RCasatta/bmp-monochrome")
    (synopsis
     "Encode and decode monochromatic bitmaps without additional dependencies, useful for QR codes.")
    (description
     "Encode and decode monochromatic bitmaps without additional dependencies, useful for QR codes.")
    (license license:expat)))

(define-public rust-qr-code-1
  (package
    (name "rust-qr-code")
    (version "1.1.0")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "qr_code" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "09krdiw69x63zs655acakr709bbz3hx56nhw4r4s8lm1gp6zn82m"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs
       (("rust-arbitrary" ,rust-arbitrary-0.4)
	("rust-bmp-monochrome" ,rust-bmp-monochrome-1)
	("rust-g2p" ,rust-g2p-0.4))
       #:cargo-development-inputs
       (("rust-hex" ,rust-hex-0.4)
	("rust-rand" ,rust-rand-0.7))))
    (home-page "https://github.com/RCasatta/qr_code")
    (synopsis
     "QR code encoder in Rust, support structured append (data in multiple qrcodes)")
    (description
     "QR code encoder in Rust, support structured append (data in multiple qrcodes)")
    (license (list license:expat license:asl2.0))))

;; pandoc rust package
(define-public rust-pandoc-0.8
  (package
    (name "rust-pandoc")
    (version "0.8.4")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "pandoc" version))
       (file-name
	(string-append name "-" version ".tar.gz"))
       (sha256
	(base32
	 "0nh6yjiwikm6v5fpifbk811ngkdf4jcjqc1q82xcshmp5r20w1l4"))))
    (build-system cargo-build-system)
    (arguments
     `(#:cargo-inputs
       (("rust-itertools" ,rust-itertools-0.8))))
    (home-page
     "https://github.com/oli-obk/rust-pandoc")
    (synopsis
     "a library API that wraps calls to the pandoc 2.x executable")
    (description
     "a library API that wraps calls to the pandoc 2.x executable")
    (license (list license:expat license:asl2.0))))

;; thiserror version 1.0.24 (has 1.0.22 in store)
(define-public rust-thiserror-impl-1-0-24
  (package
    (inherit rust-thiserror-impl-1)
    (name "rust-thiserror-impl-1-0-24")
    (version "1.0.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "thiserror-impl" version))
       (file-name
        (string-append name "-" version ".tar.gz"))
       (sha256
        (base32
         "1h7kh6rr4vsm79dmv8qk8drhh2if3zyxc1lqa921l96q22b1hrbp"))))))

(define-public rust-thiserror-1-0-24
  (package
    (inherit rust-thiserror-1)
    (name "rust-thiserror-1-0-24")
    (version "1.0.24")
    (source
     (origin
       (method url-fetch)
       (uri (crate-uri "thiserror" version))
       (file-name
        (string-append name "-" version ".tar.gz"))
       (sha256
        (base32
         "13m99wjikivkkwd209fgxhdprjxj17s39ldfvn1l8k89jxasdx70"))))
    (arguments
     `(#:cargo-inputs
       (("rust-thiserror-impl" ,rust-thiserror-impl-1-0-24))))))

;; Assert_fs

(define-public rust-assert-fs-1
  (package
    (inherit rust-assert-fs-0.11)
    (name "rust-assert-fs")
    (version "1.0.2")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "assert_fs" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
	   "0a4hqi7cz75yrnkyll64w356a61j4slnhi56a2wdy0424k58bi3k"))))
    (arguments
     `(#:cargo-inputs
       (("rust-globwalk" ,rust-globwalk-0.8) ;; override to 0.8 not 0.5
        ("rust-predicates" ,rust-predicates-1)
        ("rust-predicates-core" ,rust-predicates-core-1)
        ("rust-predicates-tree" ,rust-predicates-tree-1)
        ("rust-tempfile" ,rust-tempfile-3))
       #:cargo-development-inputs
       (("rust-docmatic" ,rust-docmatic-0.1))))))

;; rust-pdf

(define-public rust-snafu-derive-0.6
  (package
    (name "rust-snafu-derive")
    (version "0.6.10")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "snafu-derive" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0nri7ma06g5kimpcdcm8359a55nmps5f3kcngy0j6bin7jhfy20m"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-proc-macro2" ,rust-proc-macro2-1)
         ("rust-quote" ,rust-quote-1)
         ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/shepmaster/snafu")
    (synopsis "An ergonomic error handling library")
    (description
      "An ergonomic error handling library")
    (license (list license:expat license:asl2.0))))

(define-public rust-snafu-0.6
  (package
    (name "rust-snafu")
    (version "0.6.10")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "snafu" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "19wwqxwb85pl040qk5xylj0vlznib3xzy9hcv2q0h8qv4qy2vcga"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-backtrace" ,rust-backtrace-0.3)
         ("rust-doc-comment" ,rust-doc-comment-0.3)
         ("rust-futures" ,rust-futures-0.3)
         ("rust-futures" ,rust-futures-0.1)
         ("rust-futures-core" ,rust-futures-core-0.3)
         ("rust-pin-project" ,rust-pin-project-0.4)
         ("rust-snafu-derive" ,rust-snafu-derive-0.6))))
    (home-page "https://github.com/shepmaster/snafu")
    (synopsis "An ergonomic error handling library")
    (description
      "An ergonomic error handling library")
    (license (list license:expat license:asl2.0))))

(define-public rust-pdf-derive-0.1
  (package
    (name "rust-pdf-derive")
    (version "0.1.22")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "pdf_derive" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1y5lsj79vdlpw93wivl7xvvxbbpi9k3cn5bvx26pkl3m4wk0fh3z"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-proc-macro2" ,rust-proc-macro2-1)
         ("rust-quote" ,rust-quote-1)
         ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/pdf-rs")
    (synopsis "helper for pdf-rs.")
    (description "helper for pdf-rs.")
    (license license:expat)))

(define-public rust-ordermap-0.4
  (package
    (name "rust-ordermap")
    (version "0.4.2")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "ordermap" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1m0vxmlm1x92m1ydgpddzg5mrfk3ddy8gk3r9dmpml18qrs9ch4i"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-serde" ,rust-serde-1))))
    (home-page "https://github.com/bluss/ordermap")
    (synopsis
      "A hash table with consistent order and fast iteration. NOTE: This crate was renamed to indexmap. Please use it under its new name.")
    (description
      "This package provides a hash table with consistent order and fast iteration.  NOTE: This crate was renamed to indexmap.  Please use it under its new name.")
    (license (list license:asl2.0 license:expat))))

(define-public rust-fax-derive-0.1
  (package
    (name "rust-fax-derive")
    (version "0.1.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "fax_derive" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0fas9nvz2yqidlv2my7lvp0n1p4gkjlq2ln791ik3j1dkzy7y79w"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-proc-macro2" ,rust-proc-macro2-1)
         ("rust-quote" ,rust-quote-1)
         ("rust-syn" ,rust-syn-1))))
    (home-page "https://github.com/pdf-rs/fax")
    (synopsis "Bitstream matcher for the fax crate")
    (description
      "Bitstream matcher for the fax crate")
    (license license:expat)))

(define-public rust-fax-0.1
  (package
    (name "rust-fax")
    (version "0.1.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "fax" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0xb35dd2sn3k22gfwc429dzvsvk6lqkbd04ih95asf0cncmfp7i9"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-fax-derive" ,rust-fax-derive-0.1))))
    (home-page "https://github.com/pdf-rs/fax")
    (synopsis
      "Decoder and Encoder for CCITT Group 3 and 4 bi-level image encodings used by fax machines TIFF and PDF.")
    (description
      "Decoder and Encoder for CCITT Group 3 and 4 bi-level image encodings used by fax machines TIFF and PDF.")
    (license license:expat)))

(define-public rust-deflate-0.9
  (package
    (name "rust-deflate")
    (version "0.9.1")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "deflate" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0w0ww0hrq4bjnihxgbnrri4lj5c8yzg31fyzx36fd9pvvw2vz5az"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-adler32" ,rust-adler32-1)
         ("rust-gzip-header" ,rust-gzip-header-0.3))))
    (home-page
      "https://github.com/image-rs/deflate-rs")
    (synopsis
      "A DEFLATE, zlib and gzip encoder written in rust.
")
    (description
      "This package provides a DEFLATE, zlib and gzip encoder written in rust.
")
    (license (list license:expat license:asl2.0))))

(define-public rust-block-modes-0.7
  (package
    (name "rust-block-modes")
    (version "0.7.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "block-modes" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1w3jc3n7k4xq98b9mfina4wwpg1fq1s3b0mm5whqialb7q3yi82p"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-block-padding" ,rust-block-padding-0.2)
         ("rust-cipher" ,rust-cipher-0.2))))
    (home-page
      "https://github.com/RustCrypto/block-ciphers")
    (synopsis "Block cipher modes of operation")
    (description "Block cipher modes of operation")
    (license (list license:expat license:asl2.0))))

(define-public rust-aesni-0.10
  (package
    (name "rust-aesni")
    (version "0.10.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "aesni" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1kmh07fp9hbi1aa8dr2rybbgw8vqz6hjmk34c4w7sbscx7si2bpa"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-cipher" ,rust-cipher-0.2)
         ("rust-opaque-debug" ,rust-opaque-debug-0.3))))
    (home-page
      "https://github.com/RustCrypto/block-ciphers")
    (synopsis
      "AES (Rijndael) block ciphers implementation using AES-NI")
    (description
      "AES (Rijndael) block ciphers implementation using AES-NI")
    (license (list license:expat license:asl2.0))))

(define-public rust-aes-soft-0.6
  (package
    (name "rust-aes-soft")
    (version "0.6.4")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "aes-soft" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0wj0fi2pvmlw09yvb1aqf0mfkzrfxmjsf90finijh255ir4wf55y"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-cipher" ,rust-cipher-0.2)
         ("rust-opaque-debug" ,rust-opaque-debug-0.3))))
    (home-page
      "https://github.com/RustCrypto/block-ciphers")
    (synopsis
      "AES (Rijndael) block ciphers bit-sliced implementation")
    (description
      "AES (Rijndael) block ciphers bit-sliced implementation")
    (license (list license:expat license:asl2.0))))

(define-public rust-aes-0.6
  (package
    (name "rust-aes")
    (version "0.6.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "aes" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0q85mw70mgr4glza9y9lrs9nxfa1cdcqzfk6wx0smb3623pr2hw8"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-aes-soft" ,rust-aes-soft-0.6)
         ("rust-aesni" ,rust-aesni-0.10)
         ("rust-cipher" ,rust-cipher-0.2))))
    (home-page
      "https://github.com/RustCrypto/block-ciphers")
    (synopsis
      "Pure Rust implementation of the Advanced Encryption Standard (a.k.a. Rijndael)
including support for AES in counter mode (a.k.a. AES-CTR)
")
    (description
      "Pure Rust implementation of the Advanced Encryption Standard (a.k.a.  Rijndael)
including support for AES in counter mode (a.k.a.  AES-CTR)
")
    (license (list license:expat license:asl2.0))))

(define-public rust-pdf-0.7
  (package
    (name "rust-pdf")
    (version "0.7.2")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "pdf" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1n9h8k1r02p92pn831bw2vxkv0lvbskizhypd0s3z72f5x5n8j3g"))))
    (build-system cargo-build-system)
    (arguments
      `(#:cargo-inputs
        (("rust-aes" ,rust-aes-0.6)
         ("rust-block-modes" ,rust-block-modes-0.7)
         ("rust-byteorder" ,rust-byteorder-1)
         ("rust-chrono" ,rust-chrono-0.4)
         ("rust-deflate" ,rust-deflate-0.9)
         ("rust-fax" ,rust-fax-0.1)
         ("rust-glob" ,rust-glob-0.3)
         ("rust-inflate" ,rust-inflate-0.4)
         ("rust-itertools" ,rust-itertools-0.10)
         ("rust-jpeg-decoder" ,rust-jpeg-decoder-0.1)
         ("rust-log" ,rust-log-0.4)
         ("rust-md5" ,rust-md5-0.7)
         ("rust-memmap" ,rust-memmap-0.7)
         ("rust-num-traits" ,rust-num-traits-0.2)
         ("rust-once-cell" ,rust-once-cell-1)
         ("rust-ordermap" ,rust-ordermap-0.4)
         ("rust-pdf-derive" ,rust-pdf-derive-0.1)
         ("rust-sha2" ,rust-sha2-0.9)
         ("rust-snafu" ,rust-snafu-0.6)
         ("rust-stringprep" ,rust-stringprep-0.1)
         ("rust-tempfile" ,rust-tempfile-3)
         ("rust-weezl" ,rust-weezl-0.1))))
    (home-page "https://github.com/pdf-rs")
    (synopsis "PDF reader")
    (description "PDF reader")
    (license license:expat)))

;; rqrr

(define-public rust-lru-0.6
  (package
    (name "rust-lru")
    (version "0.6.5")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "lru" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1flh9fznv65mjpbk8cisykxpzf9fmfpjiv1x7nzps7gwrm14sdqz"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-hashbrown" ,rust-hashbrown-0.9))))
    (home-page
      "https://github.com/jeromefroe/lru-rs")
    (synopsis "A LRU cache implementation")
    (description
      "This package provides a LRU cache implementation")
    (license license:expat)))

(define-public rust-g2poly-0.4
  (package
    (name "rust-g2poly")
    (version "0.4.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "g2poly" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0ys5r96dr6ywam37mxkhj6wbp9qd6l5hxjc9gvq0g9gwi1w7cdz8"))))
    (build-system cargo-build-system)
    (arguments `(#:skip-build? #t))
    (home-page "https://github.com/WanzenBug/g2p")
    (synopsis
      "Primitive implementation of polynomials over the field GF(2)
")
    (description
      "Primitive implementation of polynomials over the field GF(2)
")
    (license (list license:expat license:asl2.0))))

(define-public rust-g2gen-0.4
  (package
    (name "rust-g2gen")
    (version "0.4.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "g2gen" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0m8590wqm57p4dik77s5x8lcydas9kwkzllc71f8r033djqh1h9g"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-g2poly" ,rust-g2poly-0.4)
         ("rust-proc-macro2" ,rust-proc-macro2-0.4)
         ("rust-quote" ,rust-quote-0.6)
         ("rust-syn" ,rust-syn-0.15))))
    (home-page "https://github.com/WanzenBug/g2p")
    (synopsis
      "A macro to create types that implement fast finite field arithmetic.
")
    (description
      "This package provides a macro to create types that implement fast finite field arithmetic.
")
    (license (list license:expat license:asl2.0))))

(define-public rust-g2p-0.4
  (package
    (name "rust-g2p")
    (version "0.4.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "g2p" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0m97qvalz4aq68hps0m19gkh617qr2y4a3ik1d1fbjr94rivq2dz"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-g2gen" ,rust-g2gen-0.4)
         ("rust-g2poly" ,rust-g2poly-0.4))))
    (home-page "https://github.com/WanzenBug/g2p")
    (synopsis
      "A crate to create types that implement fast finite field arithmetic.
")
    (description
      "This package provides a crate to create types that implement fast finite field arithmetic.
")
    (license (list license:expat license:asl2.0))))

(define-public rust-rqrr-0.3
  (package
    (name "rust-rqrr")
    (version "0.3.2")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "rqrr" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1k84hjqnsjpzmz0lqlldmmqabignnbz88c3fwzdihvsf07d672p8"))))
    (build-system cargo-build-system)
    (arguments
      `(#:cargo-inputs
        (("rust-g2p" ,rust-g2p-0.4)
         ("rust-image" ,rust-image-0.23)
         ("rust-lru" ,rust-lru-0.6))
        #:cargo-development-inputs
        (("rust-criterion" ,rust-criterion-0.3))))
    (home-page "https://github.com/WanzenBug/rqrr")
    (synopsis
      "Detect and read QR codes from any image source
")
    (description
      "Detect and read QR codes from any image source
")
    (license (list license:expat license:asl2.0))))


;; test-case@1.1.0
(define-public rust-similar-1
  (package
    (name "rust-similar")
    (version "1.3.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "similar" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1v4ara277c2s8gcv821b9392ki5zzw95brfs8vy3bcjpln4d9l8s"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-bstr" ,rust-bstr-0.2)
         ("rust-unicode-segmentation"
          ,rust-unicode-segmentation-1))))
    (home-page
      "https://github.com/mitsuhiko/similar")
    (synopsis "A diff library for Rust")
    (description
      "This package provides a diff library for Rust")
    (license license:asl2.0)))

(define-public rust-ron-0.6
  (package
    (inherit rust-ron-0.5)
    (name "rust-ron")
    (version "0.6.4")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "ron" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "07vzhbrnimz1lij0f280y624j4yzipn2404jmygs24mp7xhshkh6"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-base64" ,rust-base64-0.13)
         ("rust-bitflags" ,rust-bitflags-1)
         ("rust-indexmap" ,rust-indexmap-1)
         ("rust-serde" ,rust-serde-1))))))

(define-public rust-pest-2
  (package
    (name "rust-pest")
    (version "2.1.3")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "pest" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0lry80bm90x47nq71wxq83kjrm9ashpz4kbm92p90ysdx4m8gx0h"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-serde" ,rust-serde-1)
         ("rust-serde-json" ,rust-serde-json-1)
         ("rust-ucd-trie" ,rust-ucd-trie-0.1))))
    (home-page "https://pest-parser.github.io/")
    (synopsis "The Elegant Parser")
    (description "The Elegant Parser")
    (license (list license:expat license:asl2.0))))

(define-public rust-globset-0.4
  (package
    (name "rust-globset")
    (version "0.4.6")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "globset" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0jhy0qs5k43g8zyx1fys44kcdzjmcrwjyv9n703kj8g4y6g1cln1"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-aho-corasick" ,rust-aho-corasick-0.7)
         ("rust-bstr" ,rust-bstr-0.2)
         ("rust-fnv" ,rust-fnv-1)
         ("rust-log" ,rust-log-0.4)
         ("rust-regex" ,rust-regex-1)
         ("rust-serde" ,rust-serde-1))))
    (home-page
      "https://github.com/BurntSushi/ripgrep/tree/master/crates/globset")
    (synopsis
      "Cross platform single glob and glob set matching. Glob set matching is the
process of matching one or more glob patterns against a single candidate path
simultaneously, and returning all of the globs that matched.
")
    (description
      "Cross platform single glob and glob set matching.  Glob set matching is the
process of matching one or more glob patterns against a single candidate path
simultaneously, and returning all of the globs that matched.
")
    (license (list license:unlicense license:expat))))

(define-public rust-insta-1
  (package
    (name "rust-insta")
    (version "1.7.1")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "insta" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "0i4jdrr888h0qvfhy2b4vvswlpi2x2ghwg31ljfa9kki54db58f4"))))
    (build-system cargo-build-system)
    (arguments
      `(#:skip-build?
        #t
        #:cargo-inputs
        (("rust-backtrace" ,rust-backtrace-0.3)
         ("rust-console" ,rust-console-0.14)
         ("rust-csv" ,rust-csv-1)
         ("rust-globset" ,rust-globset-0.4)
         ("rust-lazy-static" ,rust-lazy-static-1)
         ("rust-pest" ,rust-pest-2)
         ("rust-pest-derive" ,rust-pest-derive-2)
         ("rust-ron" ,rust-ron-0.6)
         ("rust-serde" ,rust-serde-1)
         ("rust-serde-json" ,rust-serde-json-1)
         ("rust-serde-yaml" ,rust-serde-yaml-0.8)
         ("rust-similar" ,rust-similar-1)
         ("rust-toml" ,rust-toml-0.5)
         ("rust-uuid" ,rust-uuid-0.8)
         ("rust-walkdir" ,rust-walkdir-2))))
    (home-page "https://insta.rs/")
    (synopsis "A snapshot testing library for Rust")
    (description
      "This package provides a snapshot testing library for Rust")
    (license license:asl2.0)))

(define-public rust-test-case-1-1
  (package
    (inherit rust-test-case-1)
    (name "rust-test-case")
    (version "1.1.0")
    (source
      (origin
        (method url-fetch)
        (uri (crate-uri "test-case" version))
        (file-name
          (string-append name "-" version ".tar.gz"))
        (sha256
          (base32
            "1wb5917v3g1fsqv4h474zllh6ryhfrgyr7f163lds5r92bpl8q4m"))))))
