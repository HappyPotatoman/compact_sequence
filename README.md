# compact_sequence

[![CI/CD](https://github.com/HappyPotatoman/compact_sequence/actions/workflows/healthcheck.yml/badge.svg)](https://github.com/HappyPotatoman/compact_sequence/actions/workflows/healthcheck.yml)

This is meant as a attempt to reduce mostly nucleotide/peptide sequence storage space.

0.1.0: As of now the program in implemented in a way that it groups reoccuring characters into the character and the amount of successive occurences e.g. AAA -> A3. This is an extremly efficient way to compress highly repetetive sequences. In some test runs when trying on real biological data e.g. Homo sapiens insulin gene and Drosophila melanogaster genome sequence data the compressed .txt files were ~85-90% of the original file size.
The current version also supports multithreaded processing of directories.

Currently supported formats to be compressed and unpacked:

  - .txt
  - .fasta, .fna, .ffn, .faa, .frn, .fa (coming soon)
